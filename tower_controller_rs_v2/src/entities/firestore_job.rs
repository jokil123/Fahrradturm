use serde_derive::{Deserialize, Serialize};

use crate::controller_error::ControllerError;

use super::{
    firestore_box::{BoxType, FirestoreBoxId},
    firestore_user::FirestoreUserId,
};

pub type FirestoreJobId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreJob {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<FirestoreJobId>,
    pub assignment_type: JobType,
    pub error: Option<ControllerError>,
    pub box_id: Option<FirestoreBoxId>,
    pub user_id: FirestoreUserId,
    pub confirmation: Option<ConfirmType>,
    pub box_type: Option<BoxType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum JobType {
    #[default]
    Store,
    Retrieve,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JobError {
    NoFreeSlots,
    InvalidMessage,
    InvalidPermissions,
    NoSlotSpecified,
    InvalidSlot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ConfirmType {
    JobRecieved,
    JobCompleted,
}
