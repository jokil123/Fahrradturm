use rand_derive::Rand;
use serde::{Deserialize, Serialize};

use super::firestore_user::FirestoreUserId;

pub type FirestoreBoxId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirestoreBox {
    #[serde(skip_serializing)]
    #[serde(alias = "_firestore_id")]
    pub id: Option<FirestoreBoxId>,
    pub rented_by: Option<FirestoreUserId>,
    #[serde(rename = "type")]
    pub box_type: BoxType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Rand, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BoxType {
    #[default]
    Bike,
    Item,
}

// impl From<Vec<u32>> for FirestoreBoxId {}
