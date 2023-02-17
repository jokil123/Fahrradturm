use firestore::FirestoreTimestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreKey {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: String,
    pub start: FirestoreTimestamp,
    pub end: Option<FirestoreTimestamp>,
    pub retrieve_count: Option<u32>,
    pub store_count: Option<u32>,
    pub token: String,
}
