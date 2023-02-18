use firestore::{FirestoreReference, FirestoreTimestamp};
use serde::{Deserialize, Serialize};

pub type FirestoreRentalId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreRental {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<FirestoreRentalId>,
    pub tower_id: String,
    pub box_id: String,
    pub start: FirestoreTimestamp,
    pub end: Option<FirestoreTimestamp>,
}
