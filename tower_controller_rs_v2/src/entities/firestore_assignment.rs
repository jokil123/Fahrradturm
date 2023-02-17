use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreAssignment {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub assignment_type: AssignmentType,
    pub error: Option<AssignmentError>,
    pub slot: Option<Vec<u32>>,
    pub user_id: String,
    pub confirmation: Option<ConfirmType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AssignmentType {
    #[default]
    Store,
    Retrieve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssignmentError {
    NoFreeSlots,
    InvalidMessage,
    InvalidPermissions,
    NoSlotSpecified,
    InvalidSlot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConfirmType {
    JobRecieved,
    JobCompleted,
}
