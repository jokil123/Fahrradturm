use firestore::{FirestoreDb, FirestoreListenEvent, FirestoreListener, FirestoreListenerTarget};
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;
use prost_types::Timestamp;
use std::{
    sync::{
        mpsc::{self, SyncSender},
        Arc,
    },
    thread,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

use crate::hashmap_token_storage::HashMapTokenStorage;

pub struct JobScheduler {
    db: Arc<Mutex<FirestoreDb>>,
    tower_id: String,
    listener: Option<FirestoreListener<FirestoreDb, HashMapTokenStorage>>,
}

impl JobScheduler {
    pub fn new(db: FirestoreDb, tower_id: String) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            tower_id,
            listener: None,
        }
    }

    pub async fn stop(&mut self) {
        self.listener
            .take()
            .expect("No listener to stop")
            .shutdown()
            .await
            .expect("Failed to shutdown listener");
    }

    pub async fn listen(&mut self) {
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let mut listener;

        {
            let db_lock = self.db.lock().await;

            listener = db_lock
                .create_listener(HashMapTokenStorage::default())
                .await
                .expect("Failed to create listener");

            db_lock
                .fluent()
                .select()
                .from("jobs")
                .parent(
                    db_lock
                        .parent_path("towers", "5aQQXeYkP0xfW3FJxjH0")
                        .unwrap(),
                )
                .listen()
                .add_target(FirestoreListenerTarget::new(1), &mut listener)
                .unwrap();
        }

        // 🤮🤮🤮🤮🤮
        // find a way to not clone the db twice
        listener
            .start(|_| async {
                println!("got event");
                Ok(())
            })
            .await
            .expect("Failed to start listener");

        self.listener = Some(listener);
    }
}
