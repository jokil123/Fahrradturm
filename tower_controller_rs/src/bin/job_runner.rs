use tower_controller_rs::{job_scheduler::Job, tower::Tower};

#[tokio::main]
async fn main() {
    let tower = Tower::new(5, 5);
    let queue = JobQueue;

    loop {
        let job: Job = queue.next().await;
        tower.run_job(job).await;
    }
}

struct JobQueue;

impl JobQueue {
    pub async fn next(&self) -> Job {
        todo!()
    }
}
