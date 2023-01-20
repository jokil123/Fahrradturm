use std::io::Read;

use dotenv::dotenv;
use firestore::{FirestoreDb, FirestoreListenerTarget};
use tower_controller_rs::{job::Job, temp_file_token_storage::TempFileTokenStorage};

use std::sync::mpsc;

use firestore::FirestoreListenEvent;

#[tokio::main]
async fn main() {
    // load env
    dotenv().ok();

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
        .by_id_in("towers")
        .batch_listen(["5aQQXeYkP0xfW3FJxjH0"])
        .add_target(FirestoreListenerTarget::new(1), &mut listener)
        .unwrap();

    listener
        .start(|e| async move {
            match e {
                firestore::FirestoreListenEvent::DocumentChange(ref doc_change) => {
                    println!("Doc changed: {:?}", doc_change);

                    if let Some(doc) = &doc_change.document {
                        let obj: tower_controller_rs::entities::firestore_tower::FirestoreTower =
                            FirestoreDb::deserialize_doc_to::<
                                tower_controller_rs::entities::firestore_tower::FirestoreTower,
                            >(doc)
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
        .await
        .expect("Failed to start listener");

    std::io::stdin().read(&mut [1]).unwrap();

    listener
        .shutdown()
        .await
        .expect("Failed to shutdown listener");
}
