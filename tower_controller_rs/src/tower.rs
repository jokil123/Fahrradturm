use std::{collections::HashMap, sync::Arc};

use crate::storage_box::{box_location::BoxLocation, storage_box::StorageBox};

pub struct Tower {
    pub storage: HashMap<Arc<BoxLocation>, Option<StorageBox>>,
    pub storage_layout: (u32, u32),
}

impl Tower {
    pub fn new(levels: u32, boxes_per_level: u32) -> Self {
        let mut storage = HashMap::new();

        for level in 0..(levels - 1) {
            for index in 0..(boxes_per_level - 1) {
                storage.insert(Arc::new(BoxLocation { level, index }), None);
            }
        }

        Tower {
            storage,
            storage_layout: (levels, boxes_per_level),
        }
    }

    pub fn retrieve_box(&self, location: Arc<BoxLocation>) -> Option<StorageBox> {
        todo!("implement retrieve_box")
    }

    pub fn store_box(&mut self, location: Arc<BoxLocation>, box_to_store: StorageBox) {
        todo!("implement store_box")
    }

    pub fn find_available_box(&self) -> Option<Arc<BoxLocation>> {
        todo!("implement find_available_box")
    }

    pub fn find_available_storage(&self) -> Option<Arc<BoxLocation>> {
        self.storage
            .iter()
            .filter(|(_, storage_box)| storage_box.is_none())
            .next()
            .map(|(location, _)| location.clone())
    }
}
