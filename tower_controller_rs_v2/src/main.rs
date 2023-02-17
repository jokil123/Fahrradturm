use std::sync::Arc;

use firestore::FirestoreDb;
use tokio::sync::Mutex;
use tower_controller_rs_v2::{assignment_scheduler::JobScheduler, database::TowerDatabase};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Starting tower controller...");

    let db = Arc::new(Mutex::new(
        TowerDatabase::new("fahrradturm", "5aQQXeYkP0xfW3FJxjH0")
            .await
            .unwrap(),
    ));

    println!("Connected to database");

    let tower = Arc::new(Mutex::new(db.lock().await.fetch_tower().await.unwrap()));

    println!("Fetched tower");

    // let mut scheduler = JobScheduler::new(db.clone(), tower.clone()).await.unwrap();
    let mut scheduler = tower_controller_rs_v2::test_listener::JobScheduler::new(
        FirestoreDb::new("fahrradturm").await.unwrap(),
        "5aQQXeYkP0xfW3FJxjH0".to_string(),
    );

    println!("Created scheduler");

    // scheduler.listen().await.unwrap();

    scheduler.listen().await;

    println!("Started listening");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // scheduler.stop().await;

    println!("Stopped listening");
}
