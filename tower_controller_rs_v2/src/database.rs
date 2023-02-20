use firestore::{
    struct_path::paths, FirestoreDb, FirestoreListener, FirestoreListenerTarget, FirestoreTimestamp,
};

use chrono::Utc;

use crate::{
    controller_error::ControllerError,
    entities::{
        firestore_box::FirestoreBox,
        firestore_job::{ConfirmType, FirestoreJob},
        firestore_rental::FirestoreRental,
        firestore_tower::FirestoreTower,
        firestore_user::FirestoreUser,
    },
    hashmap_token_storage::HashMapTokenStorage,
    tower::{RentalStatus, Slot},
    util::{box_id_to_coords, coords_to_box_id, generate_n_dimensional_coords},
};

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TowerDatabase {
    db: FirestoreDb,
    pub tower_id: String,
    pub project_id: String,
}

impl TowerDatabase {
    pub async fn new(project_id: &str, tower_id: &str) -> Result<Self, ControllerError> {
        Ok(Self {
            db: FirestoreDb::new(project_id).await?,
            tower_id: tower_id.to_string(),
            project_id: project_id.to_string(),
        })
    }

    pub async fn has_subscription(&self, m: &FirestoreJob) -> Result<bool, ControllerError> {
        let user = self
            .db
            .fluent()
            .select()
            .by_id_in("users")
            .obj::<FirestoreUser>()
            .one(&m.user_id)
            .await?
            .ok_or(ControllerError::UserNotFound)?;

        Ok(user.subscription.is_some())
    }

    pub async fn set_error(&self, a_id: &str, err: ControllerError) -> Result<(), ControllerError> {
        self.db
            .fluent()
            .update()
            .fields(paths!(FirestoreJob::{error}))
            .in_col("jobs")
            .document_id(a_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreJob {
                error: Some(err.clone()),
                ..Default::default()
            })
            .execute::<FirestoreJob>()
            .await?;

        println!("Error set in db: {:?}", err);

        Ok(())
    }

    pub async fn set_confirm(&self, a_id: &str, con: ConfirmType) -> Result<(), ControllerError> {
        self.db
            .fluent()
            .update()
            .fields(paths!(FirestoreJob::{confirmation}))
            .in_col("jobs")
            .document_id(a_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreJob {
                confirmation: Some(con.clone()),
                ..Default::default()
            })
            .execute::<FirestoreJob>()
            .await?;

        println!("Confirmation set in db: {:?}", con);

        Ok(())
    }

    // TODO: improve this
    pub async fn set_slot(&self, a_id: &str, slot: &Vec<u32>) -> Result<(), ControllerError> {
        self.db
            .fluent()
            .update()
            // TODO: this is a hack, due to a library bug
            // .fields(paths!(FirestoreJob::{box_id}))
            .fields(["boxId"])
            .in_col("jobs")
            .document_id(a_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreJob {
                box_id: Some(coords_to_box_id(slot)),
                ..Default::default()
            })
            .execute::<FirestoreJob>()
            .await?;

        println!("Slot set in db: {:?}", slot);

        Ok(())
    }

    pub async fn create_listener(
        &self,
    ) -> Result<FirestoreListener<FirestoreDb, HashMapTokenStorage>, ControllerError> {
        let mut listener = self
            .db
            .create_listener(HashMapTokenStorage::default())
            .await?;

        self.db
            .fluent()
            .select()
            .from("jobs")
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .listen()
            .add_target(FirestoreListenerTarget::new(1), &mut listener)?;

        Ok(listener)
    }

    pub async fn get_tower(
        &self,
    ) -> Result<(String, Vec<u32>, HashMap<Vec<u32>, Slot>), ControllerError> {
        let f_tower = self
            .db
            .fluent()
            .select()
            .by_id_in("towers")
            .obj::<FirestoreTower>()
            .one(&self.tower_id)
            .await?
            .ok_or(ControllerError::TowerNotFound)?;

        let firestore_slots = self.create_boxes(&f_tower.layout).await?;

        let mut slots: HashMap<Vec<u32>, Slot> = HashMap::new();

        for slot_location in firestore_slots {
            let coords = box_id_to_coords(slot_location.id.as_ref().unwrap())?;

            let rental_status = match slot_location.rented_by {
                None => RentalStatus::Free,
                Some(u) => RentalStatus::Rented(u),
            };

            let slot = Slot {
                box_type: slot_location.box_type,
                rental_status: rental_status,
            };

            slots.insert(coords, slot);
        }

        Ok((self.tower_id.to_owned(), f_tower.layout, slots))
    }

    pub async fn create_boxes(
        &self,
        dimensions: &Vec<u32>,
    ) -> Result<Vec<FirestoreBox>, ControllerError> {
        let existing_docs = self
            .db
            .fluent()
            .select()
            .from("boxes")
            .parent(
                self.db
                    .parent_path("towers", self.tower_id.as_str())
                    .unwrap(),
            )
            .obj::<FirestoreBox>()
            .query()
            .await?;

        let mut required_boxes: HashSet<Vec<u32>> = generate_n_dimensional_coords(&dimensions)
            .into_iter()
            .collect();

        for doc in existing_docs {
            match box_id_to_coords(&doc.id.as_ref().unwrap()) {
                Ok(coords) => {
                    required_boxes.remove(&coords);
                }
                Err(_) => {
                    println!("Invalid box id: {}", &doc.id.as_ref().unwrap());
                    self.db
                        .fluent()
                        .delete()
                        .from("boxes")
                        .parent(
                            self.db
                                .parent_path("towers", self.tower_id.as_str())
                                .unwrap(),
                        )
                        .document_id(&doc.id.as_ref().unwrap())
                        .execute()
                        .await?;

                    println!("Deleted invalid box");
                }
            }
        }

        for coords in required_boxes {
            self.db
                .fluent()
                .insert()
                .into("boxes")
                .document_id(coords_to_box_id(&coords))
                .parent(
                    self.db
                        .parent_path("towers", self.tower_id.as_str())
                        .unwrap(),
                )
                .object(&FirestoreBox {
                    box_type: rand::random(),
                    ..Default::default()
                })
                .execute()
                .await?;
        }

        let boxes = self
            .db
            .fluent()
            .select()
            .from("boxes")
            .parent(
                self.db
                    .parent_path("towers", self.tower_id.as_str())
                    .unwrap(),
            )
            .obj::<FirestoreBox>()
            .query()
            .await?;

        Ok(boxes)
    }

    // TODO: improve this (batch delete)
    pub async fn delete_all_boxes(&self) -> Result<(), ControllerError> {
        let boxes = self
            .db
            .fluent()
            .select()
            .from("boxes")
            .parent(
                self.db
                    .parent_path("towers", self.tower_id.as_str())
                    .unwrap(),
            )
            .query()
            .await?;

        for b in boxes {
            self.db
                .fluent()
                .delete()
                .from("boxes")
                .document_id(b.name.split('/').last().unwrap())
                .parent(
                    self.db
                        .parent_path("towers", self.tower_id.as_str())
                        .unwrap(),
                )
                .execute()
                .await?;
        }

        Ok(())
    }

    pub async fn new_rental(
        &self,
        user_id: &str,
        box_location: &Vec<u32>,
    ) -> Result<String, ControllerError> {
        let box_id = coords_to_box_id(box_location);

        let storage_box = self
            .db
            .fluent()
            .select()
            .by_id_in("boxes")
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .obj::<FirestoreBox>()
            .one(&box_id)
            .await?
            .ok_or(ControllerError::NoBoxAtLocation)?;

        if storage_box.rented_by.is_some() {
            return Err(ControllerError::BoxOccupied);
        }

        let a = self
            .db
            .fluent()
            .update()
            // paths! macro is not working here because of the renamed field
            .fields(["rentedBy"])
            .in_col("boxes")
            .document_id(&box_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreBox {
                rented_by: Some(user_id.to_string()),
                ..Default::default()
            })
            .execute::<FirestoreBox>()
            .await?;

        println!("Box updated: {:?}", a);

        let rental = FirestoreRental {
            // box_ref: FirestoreReference(format!("/towers/{}/boxes/{}", &self.tower_id, &box_id)),
            box_id: box_id.clone(),
            tower_id: self.tower_id.clone(),
            start: FirestoreTimestamp(Utc::now()),
            ..Default::default()
        };

        let rental = self
            .db
            .fluent()
            .insert()
            .into("rentals")
            .generate_document_id()
            .parent(self.db.parent_path("users", user_id).unwrap())
            .object(&rental)
            .execute::<FirestoreRental>()
            .await?;

        println!("Rental created: {:?}", rental);

        Ok(rental.id.unwrap())
    }

    pub async fn finish_rental(
        &self,
        user_id: &str,
        box_location: &Vec<u32>,
    ) -> Result<(), ControllerError> {
        let box_id = coords_to_box_id(box_location);

        let storage_box = self
            .db
            .fluent()
            .select()
            .by_id_in("boxes")
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .obj::<FirestoreBox>()
            .one(&box_id)
            .await?
            .ok_or(ControllerError::NoBoxAtLocation)?;

        println!("Got box");

        if storage_box.rented_by.ok_or(ControllerError::BoxNotRented)? != user_id {
            println!("Box not rented by user");
            return Err(ControllerError::BoxNotRentedByUser);
        }

        // TODO: This function seems broken, I will use a workaround
        // let rentals = self
        //     .db
        //     .fluent()
        //     .select()
        //     .from("rentals")
        //     .parent(self.db.parent_path("users", user_id).unwrap())
        //     .filter(|q| {
        //         q.for_all([
        //             // TODO: this is due to a bug in the library
        //             q.field("towerId").eq(&self.tower_id),
        //             q.field("boxId").eq(&box_id),
        //             q.field(path!(FirestoreRental::end)).is_null(),
        //         ])
        //     })
        //     .obj::<FirestoreRental>()
        //     .query()
        //     .await?;

        let rentals = self
            .db
            .fluent()
            .select()
            .from("rentals")
            .parent(self.db.parent_path("users", user_id).unwrap())
            .obj::<FirestoreRental>()
            .query()
            .await?
            .into_iter()
            .filter(|r| r.tower_id == self.tower_id && r.box_id == box_id && r.end.is_none())
            .collect::<Vec<FirestoreRental>>();

        println!("Got rentals");

        if rentals.len() != 1 {
            println!("Invalid rental (multiple or none)");
            println!("{:#?}", rentals);
            return Err(ControllerError::InvalidRental);
        }

        let rental = rentals.first().unwrap();

        self.db
            .fluent()
            .update()
            // paths! macro is not working here because of the renamed field
            .fields(["rentedBy"])
            // .fields(paths!(FirestoreBox::{rented_by}))
            .in_col("boxes")
            .document_id(&box_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreBox {
                rented_by: None,
                ..Default::default()
            })
            .execute::<FirestoreBox>()
            .await?;

        println!("Box updated");

        let rental = self
            .db
            .fluent()
            .update()
            .fields(paths!(FirestoreRental::{end}))
            .in_col("rentals")
            .document_id(rental.id.as_ref().unwrap())
            .parent(self.db.parent_path("users", user_id).unwrap())
            .object(&FirestoreRental {
                end: Some(FirestoreTimestamp(Utc::now())),
                ..Default::default()
            })
            .execute::<FirestoreRental>()
            .await?;

        println!("Rental updated: {:?}", rental);

        Ok(())
    }
}
