use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use clone_all::clone_all;

use firestore::{FirestoreDb, FirestoreListenEvent, FirestoreListenerTarget};

use crate::{
    storage_box::{box_location::BoxLocation, box_type::BoxType},
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

    pub fn listen(&mut self) {
        self.thread_handle = Some({
            let sender = self.sender.clone();
            thread::spawn(move || {
                sender
                    .lock()
                    .expect("Failed to lock sender")
                    .send(Job {
                        created_by: "goofy ah mf".to_string(),
                        task: Task::Store(BoxType::Bicycle),
                    })
                    .expect("Failed to send job");
            })
        });
        // todo!();
    }

    pub fn stop(&mut self) {
        self.thread_handle
            .take()
            .expect("No thread handle")
            .join()
            .expect("Failed to join thread");
    }

    //     pub async fn listen(&self) -> Receiver<Job> {
    //         let (sender, reciever) = mpsc::channel::<Job>();

    //         let mut listener = self.db.create_listener(TempFileTokenStorage).await.unwrap();

    //         self.db
    //             .fluent()
    //             .select()
    //             .by_id_in("towers")
    //             .batch_listen([self.tower_id.to_owned()])
    //             .add_target(FirestoreListenerTarget::new(1), &mut listener)
    //             .unwrap();

    //         listener
    //             .start(|r| async move {
    //                 match r {
    //             // FirestoreListenEvent::
    //         }
    //             })
    //             .await;

    //         reciever
    //     }
}

pub struct Job {
    pub created_by: String,
    pub task: Task,
}

pub enum Task {
    Store(BoxType),
    /// Retrieve a Box from the tower
    Retrieve(BoxLocation),
}
