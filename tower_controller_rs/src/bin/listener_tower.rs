use dotenv::dotenv;
use firestore::*;
use rand::{self, Rng};

use std::{
    io::Read,
    time::{Duration, Instant, SystemTime},
};
use tower_controller_rs::{
    entities::firestore_tower::FirestoreTower, hashmap_token_storage::HashMapTokenStorage,
    temp_file_token_storage::TempFileTokenStorage,
};

use prost_types::Timestamp;

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    dotenv().ok();

    let db = FirestoreDb::new(&config_env_var("PROJECT_ID")?)
        .await
        .unwrap();

    let mut listener = db.create_listener(HashMapTokenStorage::default()).await?;

    db.fluent()
        .select()
        .from("jobs")
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .listen()
        .add_target(FirestoreListenerTarget::new(rand::random()), &mut listener)
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
                        println!("{:#?}", doc)
                    }
                }
                FirestoreListenEvent::DocumentDelete(_) => println!("Doc deleted"),
                // FirestoreListenEvent::DocumentRemove(_) => println!("Doc removed"),
                // FirestoreListenEvent::Filter(_) => println!("Filter"),
                // FirestoreListenEvent::TargetChange(_) => println!("Target changed"),
                _ => {}
            }

            Ok(())
        })
        .await?;
    std::io::stdin().read(&mut [1])?;

    listener.shutdown().await?;

    Ok(())
}

fn timestamp_to_duration(timestamp: Timestamp) -> Duration {
    Duration::from_secs(timestamp.seconds as u64) + Duration::from_nanos(timestamp.nanos as u64)
}
