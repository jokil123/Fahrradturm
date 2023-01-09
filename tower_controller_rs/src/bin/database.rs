use std::collections::HashMap;

use firestore::*;
use serde::{Deserialize, Serialize};

use chrono::{Date, DateTime, Utc};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let project_id: String = std::env::var("PROJECT_ID").unwrap();
    let db = FirestoreDb::new(project_id).await.unwrap();

    let t: Option<UserTest> = db
        .fluent()
        .select()
        .by_id_in("test")
        .obj()
        .one("eP9ddXSWK9gC4DfvXrzk")
        .await
        .unwrap();

    println!("{:?}", t);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserTest {
    string: String,
    number: i32,
    boolean: bool,
    map: HashMap<String, String>,
    array: Vec<bool>,
    null: (),
    timestamp: DateTime<Utc>,
    geo_point: Vec<u8>,
    reference: String,
}
