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
        .from("towers/5aQQXeYkP0xfW3FJxjH0/jobs")
        .listen()
        .add_target(FirestoreListenerTarget::new(1), &mut listener)
        .unwrap();

    listener
        .start(|event| async move {
            match event {
                FirestoreListenEvent::DocumentChange(ref doc_change) => {
                    println!("Doc changed: {:?}", doc_change);

                    if let Some(doc) = &doc_change.document {
                        let obj: FirestoreTower =
                            FirestoreDb::deserialize_doc_to::<FirestoreTower>(doc)
                                .expect("Deserialized object");
                        println!("As object: {:?}", obj);
                    }
                }
                _ => {
                    // println!("Received a listen response event to handle: {:?}", event);
                }
            }

            Ok(())
        })
        .await?;
    std::io::stdin().read(&mut [1])?;

    Ok(())
}
