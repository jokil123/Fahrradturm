use std::{
    error::Error,
    io::Read,
    sync::Arc,
    time::{Duration, SystemTime},
};

use clone_all::clone_all;
use futures::TryFutureExt;
use std::future::Future;
use tokio::sync::Mutex;

use dotenv::dotenv;
use firestore::{errors::FirestoreError, FirestoreDb, FirestoreListener, FirestoreListenerTarget};
use gcloud_sdk::google::firestore::v1::{listen_response::ResponseType, Document};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use tower_controller_rs::{
    assignment::{Assignment, AssignmentStatus, AssignmentType, JobSchedulerError},
    hashmap_token_storage::HashMapTokenStorage,
    job::Job,
    temp_file_token_storage::TempFileTokenStorage,
};

use std::sync::mpsc;

use firestore::FirestoreListenEvent;

#[tokio::main]
async fn main() {
    // load env
    dotenv().ok();

    let start_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let (sender, reciever) = mpsc::channel::<Job>();

    let db = Arc::new(Mutex::new(
        FirestoreDb::new(&std::env::var("PROJECT_ID").expect("PROJECT_ID is not set"))
            .await
            .expect("Failed to create FirestoreDb"),
    ));

    let mut listener;

    {
        let db_lock = db.lock().await;
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

    // listener
    //     .start({
    //         clone_all!(db);
    //         move |e: ResponseType| {
    //             clone_all!(db);
    //             async move { listener_callback(e, db).await }
    //         }
    //     })
    //     .await
    //     .expect("Failed to start listener");

    listener
        .start(move |e: ResponseType| {
            clone_all!(db);
            listener_callback(e, db, start_time)
        })
        .await
        .expect("Failed to start listener");

    std::io::stdin().read(&mut [1]).unwrap();

    listener
        .shutdown()
        .await
        .expect("Failed to shutdown listener");
}

async fn listener_callback(
    response: ResponseType,
    db: Arc<Mutex<FirestoreDb>>,
    start_time: Duration,
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

            let new_ass;

            if doc.create_time == doc.update_time {
                new_ass = handle_event(EventType::JobCreated, ass);
                println!("Job Created");
            } else {
                new_ass = handle_event(EventType::JobUpdated, ass);
                println!("Job Updated");
            }

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
                .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
                .object(&new_ass)
                .execute::<Assignment>()
                .await
                .unwrap();
        }
        _ => {}
    }

    Ok(())
}

enum EventType {
    JobCreated,
    JobUpdated,
}

fn handle_event(event: EventType, mut ass: Assignment) -> Option<Assignment> {
    match event {
        EventType::JobCreated => match ass.assignment_status {
            AssignmentStatus::New => {
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
