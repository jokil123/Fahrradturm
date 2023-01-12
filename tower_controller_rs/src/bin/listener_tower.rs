use dotenv::dotenv;
use firestore::*;

use std::io::Read;
use tower_controller_rs::{
    entities::firestore_tower::FirestoreTower, temp_file_token_storage::TempFileTokenStorage,
};

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let db = FirestoreDb::new(&config_env_var("PROJECT_ID")?)
        .await
        .unwrap();

    let mut listener = db.create_listener(TempFileTokenStorage).await?;

    // db.fluent()
    //     .select()
    //     .by_id_in("towers")
    //     .batch_listen(["5aQQXeYkP0xfW3FJxjH0"])
    //     .add_target(FirestoreListenerTarget::new(1), &mut listener)
    //     .unwrap();

    db.fluent()
        .select()
        .from("jobs")
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .listen()
        .add_target(FirestoreListenerTarget::new(1), &mut listener)
        .unwrap();

    listener
        .start(|event| async move {
            match event {
                FirestoreListenEvent::DocumentChange(c) => {
                    let doc = c.document.unwrap();

                    if doc.create_time == doc.update_time {
                        println!("Doc created");
                    } else {
                        println!("Doc updated");
                    }
                }
                FirestoreListenEvent::DocumentDelete(_) => println!("Doc deleted"),
                FirestoreListenEvent::DocumentRemove(_) => println!("Doc removed"),
                FirestoreListenEvent::Filter(_) => println!("Filter"),
                FirestoreListenEvent::TargetChange(_) => println!("Target changed"),
            }

            Ok(())
        })
        .await?;
    std::io::stdin().read(&mut [1])?;

    Ok(())
}
