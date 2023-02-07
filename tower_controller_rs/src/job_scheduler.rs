use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
};

use firestore::{FirestoreDb, FirestoreListenEvent, FirestoreListenerTarget};

use crate::{
    job::{Job, Task},
    storage_box::box_type::BoxType,
    temp_file_token_storage::TempFileTokenStorage,
};

pub struct JobScheduler {
    db: FirestoreDb,
    tower_id: String,
    sender: Arc<Mutex<Sender<Job>>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl JobScheduler {
    pub fn new(db: FirestoreDb, tower_id: String, sender: Sender<Job>) -> Self {
        Self {
            db,
            tower_id,
            sender: Arc::new(Mutex::new(sender)),
            thread_handle: None,
        }
    }

    pub fn listen_mock(&mut self) {
        self.thread_handle = Some({
            let sender = self.sender.clone();
            thread::spawn(move || {
                let sender_lock = sender.lock().expect("Failed to lock sender");

                sender_lock
                    .send(Job {
                        created_by: "goofy ah mf".to_string(),
                        task: Task::Store(BoxType::Bicycle),
                    })
                    .expect("Failed to send job");
            })
        });
    }

    pub fn stop(&mut self) {
        self.thread_handle
            .take()
            .expect("No thread handle")
            .join()
            .expect("Failed to join thread");
    }

    pub async fn listen(&self) {
        let mut listener = self.db.create_listener(TempFileTokenStorage).await.unwrap();

        self.db
            .fluent()
            .select()
            .from("jobs")
            .parent(
                self.db
                    .parent_path("towers", "5aQQXeYkP0xfW3FJxjH0")
                    .unwrap(),
            )
            .listen()
            .add_target(FirestoreListenerTarget::new(1), &mut listener)
            .unwrap();

        listener
            .start(|r| async move {
                handle_listen_event(r);
                Ok(())
            })
            .await
            .unwrap();
    }
}

pub fn handle_listen_event(event: FirestoreListenEvent) {
    match event {
        FirestoreListenEvent::DocumentChange(c) => {
            let doc = c.document.unwrap();

            if doc.create_time == doc.update_time {
                println!("Doc created");
            } else {
                println!("Doc updated");
            }
        }
        FirestoreListenEvent::DocumentDelete(_) => println!("Doc deleted"),
        FirestoreListenEvent::DocumentRemove(_) => println!("Doc removed"),
        FirestoreListenEvent::Filter(_) => println!("Filter"),
        FirestoreListenEvent::TargetChange(_) => println!("Target changed"),
    }
}
