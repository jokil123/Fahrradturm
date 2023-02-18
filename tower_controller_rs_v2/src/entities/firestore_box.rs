use serde::{Deserialize, Serialize};

pub type FirestoreBoxId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreBox {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub rented_by: Option<String>,
    pub box_type: BoxType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum BoxType {
    #[default]
    Bike,
    Item,
}
