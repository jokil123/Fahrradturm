use dotenv::dotenv;
use firestore::FirestoreDb;
use tower_controller_rs::{assignment::Assignment, storage_box::box_type::BoxType};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = FirestoreDb::new(&std::env::var("PROJECT_ID").expect("PROJECT_ID is not set"))
        .await
        .expect("Failed to create FirestoreDb");

    // randomly create Assignment
    let mut ass: Assignment = Assignment::default();

    ass.doc_id = Some("Amogus".to_string());
    ass.box_type = Some(BoxType::Bicycle);

    let a = db
        .fluent()
        .insert()
        .into("jobs")
        .generate_document_id()
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .object(&ass)
        .execute::<Assignment>()
        .await
        .unwrap();

    let b = db
        .fluent()
        .select()
        .by_id_in("jobs")
        .parent(db.parent_path("towers", "5aQQXeYkP0xfW3FJxjH0").unwrap())
        .obj::<Assignment>()
        .one(a.doc_id.unwrap())
        .await
        .unwrap();

    println!("{:#?}", b);
}
