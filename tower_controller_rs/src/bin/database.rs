use firestore::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let project_id: String = std::env::var("PROJECT_ID").unwrap();
    let db = FirestoreDb::new(project_id).await.unwrap();

    let t: Option<FirestoreTower> = db
        .fluent()
        .select()
        .by_id_in("towers")
        .obj()
        .one("5aQQXeYkP0xfW3FJxjH0")
        .await
        .unwrap();

    println!("{:?}", t);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FirestoreTower {
    location: FirestoreLatLng,
    name: String,
    #[serde(rename = "retrieveQueue")]
    retrieve_queue: Vec<String>,
    #[serde(rename = "storeQueue")]
    store_queue: Vec<String>,
}
