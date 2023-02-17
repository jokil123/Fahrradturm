use dotenv::dotenv;
use firestore::FirestoreDb;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = FirestoreDb::new("fahrradturm")
        .await
        .expect("Failed to create FirestoreDb");

    let docs = db
        .fluent()
        .select()
        .from("jobs")
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .query()
        .await
        .unwrap();

    println!("{:?}", docs);

    for doc in docs {
        let delete = db
            .fluent()
            .delete()
            .from("jobs")
            .document_id(doc.name.split('/').last().unwrap())
            .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
            .execute()
            .await
            .unwrap();

        println!("deleted: {:?}", delete);
    }
}
