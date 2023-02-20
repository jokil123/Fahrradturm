use firestore::{FirestoreDb, FirestoreListener};
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;
use std::{sync::Arc, time::SystemTime};
use tokio::sync::Mutex;

use crate::{
    controller_error::ControllerError, database::TowerDatabase, handle_message::handle_message,
    hashmap_token_storage::HashMapTokenStorage, tower::Tower,
};

#[derive(Debug)]
pub struct AssignmentScheduler {
    db: Arc<TowerDatabase>,
    tower: Arc<Mutex<Tower>>,
    listener: FirestoreListener<FirestoreDb, HashMapTokenStorage>,
}

impl AssignmentScheduler {
    pub async fn new(
        db: Arc<TowerDatabase>,
        tower: Arc<Mutex<Tower>>,
    ) -> Result<Self, ControllerError> {
        Ok(Self {
            db: db.clone(),
            tower,
            listener: db.create_listener().await?,
        })
    }

    pub async fn stop(&mut self) {
        self.listener
            .shutdown()
            .await
            .expect("Failed to shutdown listener");
    }

    pub async fn listen(&mut self) -> Result<(), ControllerError> {
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        // let id = self.tower.lock().await.id.clone();

        // 🤮🤮🤮🤮🤮
        // find a way to not clone the db twice
        self.listener
            .start({
                let db = self.db.clone();
                let tower = self.tower.clone();
                move |e: ResponseType| handle_message(e, start_time, db.clone(), tower.clone())
            })
            .await?;

        Ok(())
    }
}
