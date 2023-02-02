use firestore::errors::FirestoreError;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum JobSchedulerError {
    #[error("General error")]
    Err,
    #[error("Malformed assignment document {0}")]
    DeserializeError(FirestoreError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignment {
    #[serde(alias = "_firestore_id")]
    pub doc_id: Option<String>,
    pub tower: String,
    pub user: String,
    #[serde(rename = "assignmentType")]
    pub assignment_type: AssignmentType,
    pub status: AssignmentStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AssignmentType {
    #[serde(rename = "store")]
    Store,
    #[serde(rename = "retrieve")]
    Retrieve,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AssignmentStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error,
}
