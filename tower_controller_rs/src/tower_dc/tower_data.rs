use std::{collections::HashMap, sync::Arc};

use crate::storage_box::{box_location::BoxLocation, storage_box::StorageBox};

pub struct TowerData {
    pub storage: HashMap<Arc<BoxLocation>, Option<StorageBox>>,
    pub storage_layout: (u32, u32),
}
