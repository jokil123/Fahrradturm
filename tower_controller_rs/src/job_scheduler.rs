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

use crate::{
    assignment::{Assignment, AssignmentStatus, AssignmentType},
    hashmap_token_storage::HashMapTokenStorage,
    job::{self, Job, Task},
};

pub struct JobScheduler {
    db: Arc<Mutex<FirestoreDb>>,
    tower_id: String,
    sender: SyncSender<Job>,
    listener: Option<FirestoreListener<FirestoreDb, HashMapTokenStorage>>,
}

impl JobScheduler {
    pub fn new(db: FirestoreDb, tower_id: String, sender: SyncSender<Job>) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            tower_id,
            sender: sender,
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
            .start({
                let db = self.db.clone();
                let sender = self.sender.clone();
                let tower_id = self.tower_id.clone();
                move |e: ResponseType| {
                    listener_callback(e, db.clone(), start_time, sender.clone(), tower_id.clone())
                }
            })
            .await
            .expect("Failed to start listener");

        self.listener = Some(listener);
    }
}

async fn listener_callback(
    response: ResponseType,
    db: Arc<Mutex<FirestoreDb>>,
    start_time: Duration,
    sender: mpsc::SyncSender<Job>,
    tower_id: String,
) -> std::result::Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    match response {
        FirestoreListenEvent::DocumentChange(c) => {
            let doc = c.document.unwrap();

            if timestamp_to_duration(doc.update_time.clone().unwrap()) < start_time {
                return Ok(());
            }

            let ass;

            match FirestoreDb::deserialize_doc_to::<Assignment>(&doc) {
                Ok(a) => ass = a,
                Err(e) => {
                    println!("Failed to deserialize doc to Assignment: {}", e);
                    return Ok(());
                }
            }

            handle_event(EventType::JobUpdated, ass, sender);

            println!("{:#?}", new_ass);

            let Some(new_ass) = new_ass else {
                return Ok(());
            };

            let db = db.lock().await;

            println!("Sending Changes to Firestore...");

            db.fluent()
                .update()
                .in_col("jobs")
                .document_id(new_ass.doc_id.clone().unwrap())
                .parent(db.parent_path("towers", tower_id).unwrap())
                .object(&new_ass)
                .execute::<Assignment>()
                .await
                .unwrap();
        }
        _ => {}
    }

    Ok(())
}

fn handle_event(mut ass: Assignment, sender: SyncSender<Job>) -> Option<Assignment> {
    match event {
        EventType::JobCreated => match ass.assignment_status {
            AssignmentStatus::New => {
                let job = job_from_assignment(&ass);
                sender.send(job?).unwrap();

                ass.assignment_status = AssignmentStatus::Ongoing;
            }
            _ => {
                ass.assignment_status = AssignmentStatus::Error;
            }
        },
        EventType::JobUpdated => None?,
    }

    // TODO: check user and tower id

    Some(ass)
}

fn timestamp_to_duration(timestamp: Timestamp) -> Duration {
    Duration::from_secs(timestamp.seconds as u64) + Duration::from_nanos(timestamp.nanos as u64)
}

fn job_from_assignment(ass: &Assignment) -> Option<Job> {
    let task = match ass.assignment_type {
        AssignmentType::Store => Task::Store(ass.box_type?),
        // TODO: change this to not require a location
        AssignmentType::Retrieve => Task::Retrieve(todo!()),
    };

    Some(Job {
        created_by: ass.user.clone(),
        task,
    })
}
