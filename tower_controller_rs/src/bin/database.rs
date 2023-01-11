use firestore::*;
use serde::{Deserialize, Serialize};
use tower_controller_rs::entities::firestore_tower::FirestoreTower;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let project_id: String = std::env::var("PROJECT_ID").unwrap();
    let db = FirestoreDb::new(project_id).await.unwrap();

    // let t: Option<FirestoreTower> = db
    //     .fluent()
    //     .select()
    //     .by_id_in("towers")
    //     .obj()
    //     .one("5aQQXeYkP0xfW3FJxjH0")
    //     .await
    //     .unwrap();

    let t: Vec<FirestoreTest> = db
        .fluent()
        .select()
        .from("test")
        .obj()
        .query()
        .await
        .unwrap();

    println!("{:?}", t);
}

#[derive(Debug, Serialize, Deserialize)]
struct FirestoreTest;
