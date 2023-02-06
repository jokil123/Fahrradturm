use std::{
    error::Error,
    io::Read,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use clone_all::clone_all;
use std::future::Future;

use dotenv::dotenv;
use firestore::{errors::FirestoreError, FirestoreDb, FirestoreListener, FirestoreListenerTarget};
use gcloud_sdk::google::firestore::v1::{listen_response::ResponseType, Document};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use tower_controller_rs::{
    assignment::{Assignment, AssignmentStatus, AssignmentType, JobSchedulerError},
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
        let db_lock = db.lock().unwrap();
        listener = db_lock
            .create_listener(TempFileTokenStorage)
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

    listener
        .start(listener_callback)
        .await
        .expect("Failed to start listener");

    std::io::stdin().read(&mut [1]).unwrap();

    listener
        .shutdown()
        .await
        .expect("Failed to shutdown listener");
}

async fn listener_callback(
    e: ResponseType,
) -> std::result::Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    // let db = FirestoreDb::new(&std::env::var("PROJECT_ID").expect("PROJECT_ID is not set"))
    //     .await
    //     .expect("Failed to create FirestoreDb");

    let ass = Assignment::default();

    let updated_assignment = db
        .fluent()
        .update()
        .in_col("jobs")
        .document_id("aaaaaaaaaaaaaaaaaaaaaaaaa")
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .object(&ass)
        .execute::<Assignment>();

    Ok(())
}

// {
//             clone_all!(db);
//             move |e| async move {
//                 // handle_listener_event(e, start_time, db).await?;

//                 let db = db.lock().unwrap();
//                 let updated_assignment: Assignment = db
//                     .fluent()
//                     .update()
//                     .in_col("jobs")
//                     .document_id("aaaaaaaaaaaaaaaaaaaaaaaaa")
//                     .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
//                     .object(&Assignment::default())
//                     .execute()
//                     .await
//                     .unwrap();

//                 Ok(())
//             }
//         }

// Fn(FirestoreListenEvent) -> F + Send + Sync + 'static,
async fn handle_listener_event(
    response: ResponseType,
    start_time: Duration,
    db: Arc<Mutex<FirestoreDb>>,
) -> Result<(), Box<(dyn Error + Send + Sync + 'static)>> {
    match response {
        FirestoreListenEvent::DocumentChange(c) => {
            let doc = c.document.unwrap();

            if timestamp_to_duration(doc.update_time.clone().unwrap()) < start_time {
                return Ok(());
            }

            if doc.create_time == doc.update_time {
                println!("Doc created");
                match new_job(doc) {
                    Ok(ass) => {
                        println!("Job created");
                        // test(db, ass).await;

                        // let a = test(db, ass);
                    }
                    Err(e) => println!("Error: {:?}", e),
                };
            } else {
                println!("Doc updated");
            }
        }
        FirestoreListenEvent::DocumentDelete(_) => println!("Doc deleted"),
        _ => {}
    }

    Ok(())
}

fn timestamp_to_duration(timestamp: Timestamp) -> Duration {
    Duration::from_secs(timestamp.seconds as u64) + Duration::from_nanos(timestamp.nanos as u64)
}

fn new_job(doc: Document) -> Result<Assignment, JobSchedulerError> {
    let mut ass: Assignment = FirestoreDb::deserialize_doc_to::<Assignment>(&doc)
        .map_err(JobSchedulerError::DeserializeError)?;

    match ass.status {
        AssignmentStatus::New => {
            ass.status = AssignmentStatus::Ongoing;
        }
        _ => {
            ass.status = AssignmentStatus::Error;
        }
    }

    // TODO: check user and tower id

    Ok(ass)
}

// async fn test(db: Arc<Mutex<FirestoreDb>>, ass: Assignment) {
//     let db = db.lock().unwrap();

//     let updated_assignment: Assignment = db
//         .fluent()
//         .update()
//         .in_col("jobs")
//         .document_id(ass.doc_id.clone().unwrap())
//         .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
//         .object(&ass)
//         .execute()
//         .await
//         .unwrap();
// }
