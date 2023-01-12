use std::sync::mpsc::{self, Receiver};

use firestore::{FirestoreDb, FirestoreListenEvent, FirestoreListenerTarget};

use crate::{
    storage_box::box_location::BoxLocation, temp_file_token_storage::TempFileTokenStorage,
};

// pub struct JobScheduler {
//     db: FirestoreDb,
//     tower_id: String,
// }

// impl JobScheduler {
//     pub fn new(db: FirestoreDb, tower_id: String) -> Self {
//         Self { db, tower_id }
//     }

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
// }

pub struct Job {
    pub created_by: String,
    pub task: Task,
}

pub enum Task {
    Store,
    /// Retrieve a Box from the tower
    Retrieve(BoxLocation),
}
