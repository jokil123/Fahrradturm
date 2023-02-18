use firestore::FirestoreLatLng;
use serde::{Deserialize, Serialize};

pub type FirestoreTowerId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreTower {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub layout: Vec<u32>,
    pub location: FirestoreLatLng,
    pub name: String,
}
