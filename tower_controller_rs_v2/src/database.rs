use firestore::{
    errors::FirestoreError, struct_path::paths, FirestoreDb, FirestoreListener,
    FirestoreListenerTarget,
};

use crate::{
    controller_error::ControllerError,
    entities::{
        firestore_assignment::{AssignmentError, ConfirmType, FirestoreAssignment},
        firestore_box::FirestoreBox,
        firestore_tower::FirestoreTower,
        firestore_user::FirestoreUser,
    },
    hashmap_token_storage::HashMapTokenStorage,
    tower::{Slot, Tower},
};

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug)]
pub struct TowerDatabase {
    // db: FirestoreDb,
    pub db: FirestoreDb,
    tower_id: String,
    project_id: String,
}

impl TowerDatabase {
    pub async fn new(project_id: &str, tower_id: &str) -> Result<Self, ControllerError> {
        Ok(Self {
            db: FirestoreDb::new(project_id).await?,
            tower_id: tower_id.to_string(),
            project_id: project_id.to_string(),
        })
    }

    pub async fn has_subscription(&self, m: &FirestoreAssignment) -> Result<bool, ControllerError> {
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

    pub async fn set_error(&self, a_id: &str, err: AssignmentError) -> Result<(), ControllerError> {
        self.db
            .fluent()
            .update()
            .fields(paths!(FirestoreAssignment::{error}))
            .in_col("jobs")
            .document_id(a_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreAssignment {
                error: Some(err.clone()),
                ..Default::default()
            })
            .execute::<FirestoreAssignment>()
            .await?;

        println!("Error set in db: {:?}", err);

        Ok(())
    }

    pub async fn set_confirm(&self, a_id: &str, con: ConfirmType) -> Result<(), ControllerError> {
        self.db
            .fluent()
            .update()
            .fields(paths!(FirestoreAssignment::{confirmation}))
            .in_col("jobs")
            .document_id(a_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreAssignment {
                confirmation: Some(con.clone()),
                ..Default::default()
            })
            .execute::<FirestoreAssignment>()
            .await?;

        println!("Confirmation set in db: {:?}", con);

        Ok(())
    }

    // TODO: improve this
    pub async fn set_slot(&self, a_id: &str, slot: &Vec<u32>) -> Result<(), ControllerError> {
        self.db
            .fluent()
            .update()
            .fields(paths!(FirestoreAssignment::{slot}))
            .in_col("jobs")
            .document_id(a_id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&FirestoreAssignment {
                slot: Some(slot.clone()),
                ..Default::default()
            })
            .execute::<FirestoreAssignment>()
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

    pub async fn fetch_tower(&self) -> Result<Tower, ControllerError> {
        let f_tower = self
            .db
            .fluent()
            .select()
            .by_id_in("towers")
            .obj::<FirestoreTower>()
            .one(&self.tower_id)
            .await?
            .ok_or(ControllerError::TowerNotFound)?;

        let slot_locations = self.create_boxes(&f_tower.layout).await?;

        let slots: HashMap<Vec<u32>, Option<Slot>> = slot_locations
            .into_iter()
            .map(|s| (s, Some(Slot::default())))
            .collect();

        Ok(Tower {
            id: self.tower_id.clone(),
            retrieved_slot: None,
            slots: slots,
            layout: f_tower.layout,
        })
    }

    pub async fn create_boxes(
        &self,
        dimensions: &Vec<u32>,
    ) -> Result<Vec<Vec<u32>>, ControllerError> {
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
                    // TODO: Delete invalid box
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
                .object(&FirestoreBox::default())
                .execute()
                .await?;
        }

        Ok(generate_n_dimensional_coords(&dimensions))
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
}

fn box_id_to_coords(id: &str) -> Result<Vec<u32>, ControllerError> {
    id.split(',')
        .map(|s| s.parse::<u32>().map_err(|_| ControllerError::ParseError))
        .collect()
}

fn coords_to_box_id(coords: &Vec<u32>) -> String {
    coords
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn generate_n_dimensional_coords(dim: &Vec<u32>) -> Vec<Vec<u32>> {
    match dim.len() {
        0 => vec![vec![]],
        1 => {
            let mut coords = Vec::new();
            for i in 0..dim[0] {
                coords.push(vec![i]);
            }
            coords
        }
        _ => {
            let mut coords = Vec::new();
            for i in 0..dim[0] {
                let mut sub_coords = generate_n_dimensional_coords(&dim[1..].to_vec());
                for mut c in sub_coords {
                    c.insert(0, i);
                    coords.push(c);
                }
            }
            coords
        }
    }
}
