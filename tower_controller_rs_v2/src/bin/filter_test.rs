use clone_all::clone_all;
use dotenv::dotenv;
use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::Document;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = FirestoreDb::new("fahrradturm")
        .await
        .expect("Failed to create FirestoreDb");

    // db.fluent().select().from("")
}
