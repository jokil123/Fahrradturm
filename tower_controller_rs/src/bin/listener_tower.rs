use async_trait::async_trait;
use chrono::prelude::*;
use dotenv::dotenv;
use firestore::*;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use std::io::Read;
use tower_controller_rs::entities::firestore_tower::FirestoreTower;

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    #[serde(alias = "_firestore_id")]
    doc_id: Option<String>,
    some_id: String,
    some_string: String,
    some_num: u64,

    #[serde(with = "firestore::serialize_as_timestamp")]
    created_at: DateTime<Utc>,
}

const TEST_COLLECTION_NAME: &str = "test-listen";

// The file where we store the cursor/token for the event when we read the last time
const RESUME_TOKEN_FILENAME: &str = "last-read-token";

// The IDs of targets - must be different for different listener targets/listeners in case you have many instances
const TEST_TARGET_ID_BY_QUERY: FirestoreListenerTarget = FirestoreListenerTarget::new(42_i32);
const TEST_TARGET_ID_BY_DOC_IDS: FirestoreListenerTarget = FirestoreListenerTarget::new(17_i32);

#[derive(Clone)]
pub struct TempFileTokenStorage;

#[async_trait]
impl FirestoreResumeStateStorage for TempFileTokenStorage {
    async fn read_resume_state(
        &self,
        target: &FirestoreListenerTarget,
    ) -> Result<Option<FirestoreListenerTargetResumeType>, Box<dyn std::error::Error + Send + Sync>>
    {
        let target_state_file_name = format!("{}.{}.tmp", RESUME_TOKEN_FILENAME, target.value());
        let token = std::fs::read_to_string(target_state_file_name)
            .ok()
            .map(|str| {
                hex::decode(&str)
                    .map(FirestoreListenerToken::new)
                    .map(FirestoreListenerTargetResumeType::Token)
                    .map_err(|e| Box::new(e))
            })
            .transpose()?;

        Ok(token)
    }

    async fn update_resume_token(
        &self,
        target: &FirestoreListenerTarget,
        token: FirestoreListenerToken,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let target_state_file_name = format!("{}.{}.tmp", RESUME_TOKEN_FILENAME, target.value());

        Ok(std::fs::write(
            target_state_file_name,
            hex::encode(token.value()),
        )?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    let db = FirestoreDb::new(&config_env_var("PROJECT_ID")?)
        .await
        .unwrap();

    let mut listener = db.create_listener(TempFileTokenStorage).await?;

    db.fluent()
        .select()
        .from("tower")
        .listen()
        .add_target(FirestoreListenerTarget::new(1), &mut listener)?;

    db.fluent()
        .select()
        .by_id_in("tower")
        .batch_listen(["5aQQXeYkP0xfW3FJxjH0"])
        .add_target(FirestoreListenerTarget::new(2), &mut listener)?;

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
                FirestoreListenEvent::DocumentDelete(_) => println!("DocumentDelete"),
                FirestoreListenEvent::DocumentRemove(_) => println!("DocumentRemove"),
                FirestoreListenEvent::Filter(_) => println!("Filter"),
                FirestoreListenEvent::TargetChange(_) => println!("TargetChange"),
            }

            Ok(())
        })
        .await?;

    std::io::stdin().read(&mut [1])?;

    listener.shutdown().await?;

    Ok(())
}
