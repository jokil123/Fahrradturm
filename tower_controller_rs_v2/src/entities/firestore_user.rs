use serde::{Deserialize, Serialize};

pub type FirestoreUserId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreUser {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<FirestoreUserId>,
    pub name: String,
    pub email: String,
    pub subscription: Option<SubscriptionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionType {
    Free,
    Premium,
}
