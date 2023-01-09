use std::{collections::HashMap, sync::Arc};

use crate::storage_box::{
    box_location::BoxLocation, box_type::BoxType, logistic_state::LogisticState,
    rental_status::RentalStatus, storage_box::StorageBox,
};

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

    pub fn retrieve_box(&mut self, location: Arc<BoxLocation>) -> Result<StorageBox, TowerError> {
        if !self.storage.contains_key(&location) {
            return Err(TowerError::InvalidLocation);
        };

        self.storage
            .remove(&location)
            .unwrap()
            .ok_or(TowerError::BoxNotFound)
    }

    pub fn store_box(
        &mut self,
        location: Arc<BoxLocation>,
        box_to_store: StorageBox,
    ) -> Result<(), TowerError> {
        if !self.storage.contains_key(&location) {
            return Err(TowerError::InvalidLocation);
        };

        if !self.storage.get(&location).unwrap().is_none() {
            return Err(TowerError::SlotOccupied);
        };

        self.storage.insert(location, Some(box_to_store));

        Ok(())
    }

    pub fn find_available_box(
        &self,
        box_type: Option<BoxType>,
    ) -> Result<Arc<BoxLocation>, TowerError> {
        self.storage
            .iter()
            .filter(|(_, storage_box)| storage_box.is_some())
            .filter(
                |(_, storage_box)| match storage_box.as_ref().unwrap().logistic_state {
                    LogisticState::Stored(_) => true,
                    _ => false,
                },
            )
            .filter(
                |(_, storage_box)| match storage_box.as_ref().unwrap().rental_status {
                    RentalStatus::Available => true,
                    _ => false,
                },
            )
            .filter(|(_, storage_box)| match box_type.as_ref() {
                Some(box_type) => &storage_box.as_ref().unwrap().box_type == box_type,
                None => true,
            })
            .next()
            .map(|(location, _)| location.clone())
            .ok_or(TowerError::NoAvailableBox)
    }

    pub fn find_available_storage(&self) -> Result<Arc<BoxLocation>, TowerError> {
        self.storage
            .iter()
            .filter(|(_, storage_box)| storage_box.is_none())
            .next()
            .map(|(location, _)| location.clone())
            .ok_or(TowerError::NoAvailableSlot)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TowerError {
    #[error("Invalid location")]
    InvalidLocation,
    #[error("Slot occupied")]
    SlotOccupied,
    #[error("Box not found")]
    BoxNotFound,
    #[error("No available box")]
    NoAvailableBox,
    #[error("No available slot")]
    NoAvailableSlot,
}
