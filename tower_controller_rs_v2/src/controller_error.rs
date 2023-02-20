use firestore::errors::FirestoreError;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ControllerError {
    #[error("Firestore Error: {0}")]
    FirestoreError(String),
    #[error("Tower not found")]
    TowerNotFound,
    #[error("Slot not found")]
    SlotNotFound,
    #[error("No free slots")]
    NoFreeSlots,
    #[error("User not found")]
    UserNotFound,
    #[error("No box at location")]
    NoBoxAtLocation,
    #[error("No box retrieved")]
    NoBoxRetrieved,
    #[error("Box occupied")]
    BoxOccupied,
    #[error("Parse error")]
    ParseError,
    #[error("Box not rented")]
    BoxNotRented,
    #[error("Box not rented by user")]
    BoxNotRentedByUser,
    #[error("Invalid rental")]
    InvalidRental,
    #[error("Invalid message")]
    InvalidMessage,
    #[error("Invalid permissions")]
    InvalidPermissions,
    #[error("No slot specified")]
    NoSlotSpecified,
    #[error("Invalid slot")]
    InvalidSlot,
    #[error("No box type specified")]
    NoBoxTypeSpecified,
}

impl From<FirestoreError> for ControllerError {
    fn from(err: FirestoreError) -> Self {
        Self::FirestoreError(err.to_string())
    }
}

// This could work but not currently worth the effort

// #[derive(Debug, Display)]
// pub struct SerializableFirestoreError(FirestoreError);

// impl From<FirestoreError> for SerializableFirestoreError {
//     fn from(err: FirestoreError) -> Self {
//         Self(err)
//     }
// }

// impl Serialize for SerializableFirestoreError {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(&self.0.to_string())
//     }
// }

// impl Error for SerializableFirestoreError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn Error> {
//         self.source()
//     }
// }
