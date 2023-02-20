use std::{collections::HashMap, default, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    controller_error::ControllerError, database::TowerDatabase, entities::firestore_box::BoxType,
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

        Ok(Self {
            id,
            slots,
            layout,
            db,
        })
    }

    // TODO: account for slot type
    pub fn find_free_slot(&self) -> Result<Vec<u32>, ControllerError> {
        let a = self
            .slots
            .iter()
            .filter(|(k, v)| v.rental_status == RentalStatus::Free)
            .next()
            .ok_or(ControllerError::NoFreeSlots)?
            .0;

        Ok(a.clone())
    }

    // TODO: update database
    pub async fn store_object(
        &mut self,
        slot: &Vec<u32>,
        user: &str,
    ) -> Result<(), ControllerError> {
        self.rent_box(user, slot)?;
        println!("Rented box");

        self.db.new_rental(user, slot).await?;
        println!("Updated database");

        Ok(())
    }

    // TODO: update database
    pub async fn retrieve_object(
        &mut self,
        slot: &Vec<u32>,
        user: &str,
    ) -> Result<(), ControllerError> {
        self.unrent_box(slot)?;
        println!("Unrented box");

        self.db.finish_rental(user, slot).await?;
        println!("Updated database");

        Ok(())
    }

    // TODO: fix this
    pub fn slot_exists(&self, slot: &Vec<u32>) -> Result<bool, ControllerError> {
        let a = self.slots.get(slot).ok_or(ControllerError::SlotNotFound)?;

        Ok(true)
    }

    pub fn slot_rented_by_user(
        &self,
        slot_location: &Vec<u32>,
        user_id: &str,
    ) -> Result<bool, ControllerError> {
        // TODO: this function bugs out because of the initial desync of database and tower

        let slot = self
            .slots
            .get(slot_location)
            .ok_or(ControllerError::SlotNotFound)?;

        println!("Slot: {:#?}", slot);
        println!("Slot Location: {:#?}", slot_location);
        println!("Tower: {:#?}", self);

        Ok(slot.rental_status == RentalStatus::Rented(user_id.to_string()))
    }

    // TODO: update database
    pub fn rent_box(
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

        slot.rental_status = RentalStatus::Rented(user_id.to_string());

        Ok(())
    }

    // TODO: update database
    pub fn unrent_box(&mut self, slot_location: &Vec<u32>) -> Result<(), ControllerError> {
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
