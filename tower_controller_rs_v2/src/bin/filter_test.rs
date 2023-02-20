use dotenv::dotenv;
// use firestore::FirestoreDb;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let db = FirestoreDb::new("fahrradturm")
    //     .await
    //     .expect("Failed to create FirestoreDb");

    // db.fluent().select().from("")
}
