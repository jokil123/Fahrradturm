use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: String,
    pub assignment_type: AssignmentType,
    pub error: Option<AssignmentError>,
    pub slot: Option<Vec<u32>>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentType {
    Store,
    Retrieve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentError {
    NoFreeSlots,
    InvalidMessage,
    InvalidPermissions,
    NoSlotSpecified,
    InvalidSlot,
}
