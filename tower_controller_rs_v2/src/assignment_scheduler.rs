use firestore::{errors::FirestoreError, FirestoreDb, FirestoreListener, FirestoreListenerTarget};
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;
use std::{sync::Arc, time::SystemTime};
use tokio::sync::Mutex;

use crate::{
    database::TowerDatabase, handle_message::handle_message,
    hashmap_token_storage::HashMapTokenStorage, tower::Tower,
};

#[derive(Debug)]
pub struct JobScheduler {
    db: Arc<Mutex<TowerDatabase>>,
    tower: Arc<Mutex<Tower>>,
    // listener: FirestoreListener<FirestoreDb, HashMapTokenStorage>,
    listener: Option<FirestoreListener<FirestoreDb, HashMapTokenStorage>>,
}

impl JobScheduler {
    pub async fn new(
        db: Arc<Mutex<TowerDatabase>>,
        tower: Arc<Mutex<Tower>>,
    ) -> Result<Self, FirestoreError> {
        Ok(Self {
            db: db.clone(),
            tower,
            // listener: db.lock().await.create_listener().await?,
            listener: None,
        })
    }

    pub async fn stop(&mut self) {
        self.listener
            .take()
            .unwrap()
            .shutdown()
            .await
            .expect("Failed to shutdown listener");
    }

    // pub async fn listen(&mut self) -> Result<(), FirestoreError> {
    //     let start_time = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .unwrap();

    //     // let mut listener = self.db.lock().await.create_listener().await?;

    //     let db = &self.db.lock().await.db;

    //     let mut listener = db.create_listener(HashMapTokenStorage::default()).await?;

    //     db.fluent()
    //         .select()
    //         .from("jobs")
    //         .parent(
    //             db.parent_path("towers", &self.tower.lock().await.id)
    //                 .unwrap(),
    //         )
    //         .listen()
    //         .add_target(FirestoreListenerTarget::new(1), &mut listener)?;

    //     // 🤮🤮🤮🤮🤮
    //     // find a way to not clone the db twice
    //     listener
    //         // .start({
    //         //     let db = self.db.clone();
    //         //     let tower = self.tower.clone();
    //         //     move |e: ResponseType| handle_message(e, start_time, db.clone(), tower.clone())
    //         // })
    //         .start(|_| async {
    //             println!("tick");
    //             Ok(())
    //         })
    //         .await?;

    //     Ok(())
    // }
}
