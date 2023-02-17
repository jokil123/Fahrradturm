use firestore::FirestoreTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreRental {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: String,
    pub box_ref: String,
    pub start: FirestoreTimestamp,
    pub end: Option<FirestoreTimestamp>,
}
