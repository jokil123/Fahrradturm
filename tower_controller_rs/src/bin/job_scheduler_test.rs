use std::{
    io::Read,
    sync::Arc,
    time::{Duration, SystemTime},
};

use dotenv::dotenv;
use firestore::{errors::FirestoreError, FirestoreDb, FirestoreListenerTarget};
use gcloud_sdk::google::firestore::v1::Document;
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

    let mut listener = db
        .create_listener(TempFileTokenStorage)
        .await
        .expect("Failed to create listener");

    db.fluent()
        .select()
        .from("jobs")
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .listen()
        .add_target(FirestoreListenerTarget::new(1), &mut listener)
        .unwrap();

    listener
        .start(move |event| async move {
            match event {
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

                                let a = test(db, ass);
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
        })
        .await
        .expect("Failed to start listener");

    std::io::stdin().read(&mut [1]).unwrap();

    listener
        .shutdown()
        .await
        .expect("Failed to shutdown listener");
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

async fn test(db: FirestoreDb, ass: Assignment) {
    let updated_assignment: Assignment = db
        .fluent()
        .update()
        .in_col("jobs")
        .document_id(ass.doc_id.clone().unwrap())
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .object(&ass)
        .execute()
        .await
        .unwrap();
}
