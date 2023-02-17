use dotenv::dotenv;
use std::{
    env,
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

use clone_all::clone_all;

use firestore::FirestoreDb;
use tower_controller_rs::{
    display::gui_display::{DisplayMessage, GUIDisplay},
    job::Job,
    job_scheduler::JobScheduler,
    tower::Tower,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut tower = Tower::new(5, 5);

    tower.fill_dummy(0.5);

    let db = FirestoreDb::new(env::var("PROJECT_ID").expect("PROJECT_ID not set"))
        .await
        .expect("Failed to create FirestoreDb");

    let tower_id = env::var("TOWER_ID").expect("TOWER_ID not set");

    let tower = Arc::new(Mutex::new(tower));
    let (job_s, job_r) = mpsc::sync_channel::<Job>(1000);

    let mut job_scheduler = JobScheduler::new(db, tower_id, job_s);
    // job_scheduler.listen_mock();
    job_scheduler.listen().await;

    let (display_s, display_r) = mpsc::channel::<DisplayMessage>();

    {
        clone_all!(tower);
        thread::spawn(move || {
            GUIDisplay::new(display_r, tower).run();
        });
    }

    loop {
        println!("waiting for job");
        let Ok(job) = job_r.recv() else {
            break;
        };

        println!("got job, running...");
        tower
            .lock()
            .expect("Failed to lock tower")
            .run_job(job)
            .await
            .expect("Failed to run job");
        println!("job successfully finished");

        display_s
            .send(DisplayMessage::Update)
            .expect("Failed to send update to display");
    }

    job_scheduler.stop().await;

    display_s
        .send(DisplayMessage::Stop)
        .expect("Failed to send stop to display");
}
