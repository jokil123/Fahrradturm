use firestore::FirestoreLatLng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirestoreTower {
    // pub doc_id: Option<String>,
    pub location: FirestoreLatLng,
    pub name: String,
    #[serde(rename = "retrieveQueue")]
    pub retrieve_queue: Vec<String>,
    #[serde(rename = "storeQueue")]
    pub store_queue: Vec<String>,
}
