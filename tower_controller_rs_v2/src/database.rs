use firestore::{errors::FirestoreError, FirestoreDb, FirestoreListener, FirestoreListenerTarget};

use crate::{
    controller_error::ControllerError,
    entities::{
        firestore_assignment::{AssignmentError, FirestoreAssignment},
        firestore_tower::FirestoreTower,
    },
    hashmap_token_storage::HashMapTokenStorage,
    tower::{Slot, Tower},
};

use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub struct TowerDatabase {
    // db: FirestoreDb,
    pub db: FirestoreDb,
    tower_id: String,
    project_id: String,
}

impl TowerDatabase {
    pub async fn new(project_id: &str, tower_id: &str) -> Result<Self, FirestoreError> {
        Ok(Self {
            db: FirestoreDb::new(project_id).await?,
            tower_id: tower_id.to_string(),
            project_id: project_id.to_string(),
        })
    }

    pub async fn check_permissions(&self, m: &FirestoreAssignment) -> Result<(), ()> {
        todo!("check permissions in db");
    }

    pub async fn set_error(&self, a_id: &str, err: AssignmentError) {
        todo!("set error in db");
    }

    pub async fn set_confirm(&self, a_id: &str, c_type: ConfirmType) {
        todo!("set confirm in db");
    }

    pub async fn create_listener(
        &self,
    ) -> Result<FirestoreListener<FirestoreDb, HashMapTokenStorage>, FirestoreError> {
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

        let mut slots: HashMap<Vec<u32>, Option<Arc<Slot>>> = HashMap::new();

        match f_tower.layout.len() {
            2 => {
                for i in 0..f_tower.layout[0] {
                    for j in 0..f_tower.layout[1] {
                        slots.insert(vec![i, j], None);
                    }
                }
            }
            3 => {
                for i in 0..f_tower.layout[0] {
                    for j in 0..f_tower.layout[1] {
                        for k in 0..f_tower.layout[2] {
                            slots.insert(vec![i, j, k], None);
                        }
                    }
                }
            }
            _ => {
                panic!("Tower layout not supported");
            }
        }

        Ok(Tower {
            id: self.tower_id.clone(),
            retrieved_slot: None,
            slots: slots,
            layout: f_tower.layout,
        })
    }
}

pub enum ConfirmType {
    JobRecieved,
    JobCompleted,
}
