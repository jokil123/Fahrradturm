use std::{collections::HashMap, default, fmt::Display, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    controller_error::ControllerError, database::TowerDatabase, entities::firestore_box::BoxType,
    tower_display::TowerDisplay,
};

#[derive(Debug)]
pub struct Tower {
    pub id: String,
    pub slots: HashMap<Vec<u32>, Slot>,
    pub layout: Vec<u32>,
    pub db: Arc<TowerDatabase>,
}

pub type SlotLocation = Vec<u32>;

impl Tower {
    pub async fn new(db: Arc<TowerDatabase>) -> Result<Self, ControllerError> {
        let (id, layout, slots) = db.get_tower().await?;

        let tower = Self {
            id,
            slots,
            layout,
            db,
        };

        TowerDisplay::go(&tower);

        Ok(tower)
    }

    pub fn find_free_slot(&self, slot_type: BoxType) -> Result<Vec<u32>, ControllerError> {
        let a = self
            .slots
            .iter()
            .filter(|(_, v)| v.rental_status == RentalStatus::Free && v.box_type == slot_type)
            .next()
            .ok_or(ControllerError::NoFreeSlots)?
            .0;

        Ok(a.clone())
    }

    // this might not be needed
    pub async fn store_object(
        &mut self,
        slot: &Vec<u32>,
        user: &str,
    ) -> Result<(), ControllerError> {
        // button pressing logic could go here
        self.rent_box(user, slot).await?;
        TowerDisplay::go(&self);
        Ok(())
    }

    // this might not be needed
    pub async fn retrieve_object(
        &mut self,
        slot: &Vec<u32>,
        user: &str,
    ) -> Result<(), ControllerError> {
        // button pressing logic could go here
        self.unrent_box(user, slot).await?;
        TowerDisplay::go(&self);
        Ok(())
    }

    // TODO: fix this
    pub fn slot_exists(&self, slot: &Vec<u32>) -> Result<(bool), ControllerError> {
        self.slots.get(slot).ok_or(ControllerError::SlotNotFound)?;

        Ok(true)
    }

    pub fn slot_rented_by_user(
        &self,
        slot_location: &Vec<u32>,
        user_id: &str,
    ) -> Result<bool, ControllerError> {
        let slot = self
            .slots
            .get(slot_location)
            .ok_or(ControllerError::SlotNotFound)?;

        println!("Slot: {:#?}", slot);
        println!("Slot Location: {:#?}", slot_location);
        println!("Tower: {:#?}", self);

        Ok(slot.rental_status == RentalStatus::Rented(user_id.to_string()))
    }

    pub async fn rent_box(
        &mut self,
        user_id: &str,
        slot_location: &Vec<u32>,
    ) -> Result<(), ControllerError> {
        let slot = self
            .slots
            .get_mut(slot_location)
            .ok_or(ControllerError::SlotNotFound)?;

        if slot.rental_status != RentalStatus::Free {
            return Err(ControllerError::BoxOccupied);
        }

        self.db.new_rental(user_id, slot_location).await?;

        slot.rental_status = RentalStatus::Rented(user_id.to_string());

        Ok(())
    }

    pub async fn unrent_box(
        &mut self,
        user_id: &str,
        slot_location: &Vec<u32>,
    ) -> Result<(), ControllerError> {
        let slot = self
            .slots
            .get_mut(slot_location)
            .ok_or(ControllerError::SlotNotFound)?;

        match slot.rental_status {
            RentalStatus::Free => return Err(ControllerError::BoxOccupied),
            RentalStatus::Rented(_) => {
                slot.rental_status = RentalStatus::Free;
            }
        }

        self.db.finish_rental(user_id, slot_location).await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Slot {
    pub rental_status: RentalStatus,
    pub box_type: BoxType,
}

#[derive(Debug, PartialEq, Default)]
pub enum RentalStatus {
    #[default]
    Free,
    Rented(String),
    // Reserved(String),
}
