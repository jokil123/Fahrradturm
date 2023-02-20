use dotenv::dotenv;
use firestore::FirestoreDb;
use tower_controller_rs_v2::entities::firestore_job::{FirestoreJob, JobType};

use tower_controller_rs_v2::entities::firestore_job::ConfirmType;

// this is a test function that should have as little side effects as possible (only creates a job)
#[tokio::main]
async fn main() {
    dotenv().ok();

    // let rng = &mut rand::thread_rng();

    let db = FirestoreDb::new("fahrradturm")
        .await
        .expect("Failed to create FirestoreDb");

    // let user_id = db
    //     .fluent()
    //     .select()
    //     .from("users")
    //     .obj::<FirestoreUser>()
    //     .query()
    //     .await
    //     .unwrap()
    //     .choose_multiple(rng, 1)
    //     .collect::<Vec<&FirestoreUser>>()
    //     .first()
    //     .unwrap()
    //     .id
    //     .clone()
    //     .unwrap();

    let user_id = "g6LKh55wQ1WdCkglO0S5".to_string();

    println!("User ID: {}", user_id);

    // let tower_id = db
    //     .fluent()
    //     .select()
    //     .from("towers")
    //     .obj::<FirestoreTower>()
    //     .query()
    //     .await
    //     .unwrap()
    //     .choose_multiple(rng, 1)
    //     .collect::<Vec<&FirestoreTower>>()
    //     .first()
    //     .unwrap()
    //     .id
    //     .clone()
    //     .unwrap();

    let tower_id = "qtGDogFK3o9LVtCrMsbW".to_string();

    println!("Tower ID: {}", tower_id);

    let store_job = FirestoreJob {
        assignment_type: JobType::Store,
        user_id: user_id.clone(),
        box_type: Some(rand::random()),
        ..Default::default()
    };

    println!("Job: {:?}", store_job.box_type);

    let job_id = db
        .fluent()
        .insert()
        .into("jobs")
        .generate_document_id()
        .parent(db.parent_path("towers", &tower_id).unwrap())
        .object(&store_job)
        .execute::<FirestoreJob>()
        .await
        .unwrap()
        .id
        .unwrap();

    println!("Job created");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let store_job = db
        .fluent()
        .select()
        .by_id_in("jobs")
        .parent(db.parent_path("towers", &tower_id).unwrap())
        .obj::<FirestoreJob>()
        .one(job_id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(store_job.error, None);
    assert_eq!(store_job.confirmation, Some(ConfirmType::JobCompleted));
    assert!(store_job.box_id.is_some());

    println!("Job successfully completed");
    println!("Starting retrieval job");

    let retrieve_job = FirestoreJob {
        assignment_type: JobType::Retrieve,
        box_id: store_job.box_id,
        user_id: user_id.clone(),
        ..Default::default()
    };

    let retrieve_job = db
        .fluent()
        .insert()
        .into("jobs")
        .generate_document_id()
        .parent(db.parent_path("towers", &tower_id).unwrap())
        .object(&retrieve_job)
        .execute::<FirestoreJob>()
        .await
        .unwrap();

    println!("Retrieval job created");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let retrieve_job = db
        .fluent()
        .select()
        .by_id_in("jobs")
        .parent(db.parent_path("towers", &tower_id).unwrap())
        .obj::<FirestoreJob>()
        .one(retrieve_job.id.unwrap())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(retrieve_job.error, None);
    assert_eq!(retrieve_job.confirmation, Some(ConfirmType::JobCompleted));
}
