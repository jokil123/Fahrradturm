use dotenv::dotenv;
use firestore::FirestoreDb;
use tower_controller_rs_v2::entities::{
    firestore_box::BoxType,
    firestore_job::{FirestoreJob, JobType},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = FirestoreDb::new("fahrradturm")
        .await
        .expect("Failed to create FirestoreDb");

    let ass = FirestoreJob {
        id: None,
        assignment_type: JobType::Store,
        error: None,
        box_id: None,
        user_id: "UtClt1RTkOR03plKjUZpkVIWr4E2".to_string(),
        confirmation: None,
        box_type: Some(BoxType::Bike),
    };

    let a = db
        .fluent()
        .insert()
        .into("jobs")
        .generate_document_id()
        .parent(db.parent_path("towers", "qtGDogFK3o9LVtCrMsbW").unwrap())
        .object(&ass)
        .execute::<FirestoreJob>()
        .await
        .unwrap();

    let b = db
        .fluent()
        .select()
        .by_id_in("jobs")
        .parent(db.parent_path("towers", "qtGDogFK3o9LVtCrMsbW").unwrap())
        .obj::<FirestoreJob>()
        .one(a.id.unwrap())
        .await
        .unwrap();

    println!("{:#?}", b);
}
