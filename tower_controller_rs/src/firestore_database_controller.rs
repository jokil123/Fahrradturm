use firestore::{errors::FirestoreError, FirestoreDb};

use crate::assignment::{self, Assignment};

struct FirestoreDatabaseController {
    db: FirestoreDb,
    tower_id: String,
}

impl FirestoreDatabaseController {
    pub fn new(db: FirestoreDb) -> Self {
        Self { db }
    }

    pub async fn set_assignment(&self, a: Assignment) -> Result<(), FirestoreError> {
        let id = a.doc_id.clone().unwrap();

        self.db
            .fluent()
            .update()
            .in_col("jobs")
            .document_id(&id)
            .parent(self.db.parent_path("towers", &self.tower_id).unwrap())
            .object(&a)
            .execute::<Assignment>()
            .await?;

        Ok(())
    }

    // pub async fn set_error(&self, a_id: String) -> Result<(), FirestoreError> {
    //     self.db
    //         .fluent()
    //         .update()
    //         .fields(paths!(Assignment::{}))
    //         .in_col("jobs")
    //         .document_id(&a_id)
    //         .parent(self.db.parent_path("towers", &self.tower_id).unwrap()).

    //     let a = assignment::get_assignment(&self.db, &a_id).await?;

    //     let a = assignment::set_error(a);

    //     self.set_assignment(a).await?;

    //     Ok(())
    // }
}
