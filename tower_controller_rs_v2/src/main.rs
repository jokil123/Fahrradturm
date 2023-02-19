use std::sync::Arc;

use firestore::FirestoreDb;
use tokio::sync::Mutex;
use tower_controller_rs_v2::{
    assignment_scheduler::AssignmentScheduler, database::TowerDatabase, tower::Tower,
};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Starting tower controller...");

    let db = Arc::new(
        TowerDatabase::new("fahrradturm", "qtGDogFK3o9LVtCrMsbW")
            .await
            .unwrap(),
    );

    println!("Connected to database");

    let tower = Arc::new(Mutex::new(Tower::new(db.clone()).await.unwrap()));

    println!("Fetched tower");
    println!("Tower:\n{:#?}", tower.lock().await);

    let mut scheduler = AssignmentScheduler::new(db.clone(), tower.clone())
        .await
        .unwrap();

    println!("Created scheduler");

    scheduler.listen().await.unwrap();

    println!("Started listening");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    scheduler.stop().await;

    println!("Stopped listening");
}
