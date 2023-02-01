use std::{
    io::Read,
    time::{Duration, SystemTime},
};

use dotenv::dotenv;
use firestore::{FirestoreDb, FirestoreListenerTarget};
use prost_types::Timestamp;
use tower_controller_rs::{job::Job, temp_file_token_storage::TempFileTokenStorage};

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

    let db = FirestoreDb::new(&std::env::var("PROJECT_ID").expect("PROJECT_ID is not set"))
        .await
        .expect("Failed to create FirestoreDb");

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
