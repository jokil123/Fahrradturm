use std::collections::HashMap;

use crate::storage_box::{BoxLocation, StorageBox};

pub struct Tower {
    pub storage: HashMap<BoxLocation, Option<StorageBox>>,
    pub storage_layout: (i32, i32),
}

impl Tower {
    pub fn new(levels: i32, boxes_per_level: i32) -> Self {
        let mut storage = HashMap::new();

        for level in 0..(levels - 1) {
            for index in 0..(boxes_per_level - 1) {
                storage.insert(BoxLocation { level, index }, None);
            }
        }

        Tower {
            storage,
            storage_layout: (levels, boxes_per_level),
        }
    }

    pub fn retrieve_box(&self, location: BoxLocation) -> Option<StorageBox> {
        todo!("implement retrieve_box")
    }

    pub fn store_box(&mut self, location: BoxLocation, box_to_store: StorageBox) {
        todo!("implement store_box")
    }

    pub fn find_available_box(&self) -> Option<BoxLocation> {
        todo!("implement find_available_box")
    }

    pub fn find_available_storage(&self) -> Option<BoxLocation> {
        todo!("implement find_available_storage")
    }
}
