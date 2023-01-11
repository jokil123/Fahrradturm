use firestore::*;
use serde::{Deserialize, Serialize};
use tower_controller_rs::entities::firestore_tower::FirestoreTower;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let project_id: String = std::env::var("PROJECT_ID").unwrap();
    let db = FirestoreDb::new(project_id).await.unwrap();

    // db.fluent().select().by_id_in("test").batch_listen(["O0U5DUMnOLyxZXJOJpOx"]).add_target(target, listener)

    // println!("{:#?}", t);

    // db.fluent().select().by_id_in("test").one(document_id)
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreTest {
    field: (),
}
