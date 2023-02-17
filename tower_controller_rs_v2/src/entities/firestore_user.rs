use firestore::FirestoreLatLng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreUser {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: String,
    pub name: String,
    pub subscription: Option<SubscriptionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionType {
    Free,
    Premium,
}
