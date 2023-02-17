use firestore::FirestoreLatLng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirestoreTower {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: String,
    pub layout: Vec<u32>,
    pub location: FirestoreLatLng,
    pub name: String,
}
