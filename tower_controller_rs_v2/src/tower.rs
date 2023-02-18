use std::{collections::HashMap, default, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    controller_error::ControllerError, database::TowerDatabase, entities::firestore_box::BoxType,
};

#[derive(Debug)]
pub struct Tower {
    pub id: String,
    pub slots: HashMap<Vec<u32>, Option<Slot>>,
    pub retrieved_slot: Option<Slot>,
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
            retrieved_slot: None,
            layout,
            db,
        })
    }

    pub fn find_free_slot(&self) -> Result<Vec<u32>, ControllerError> {
        let a = self
            .slots
            .iter()
            .filter(|(k, v)| v.is_some() && v.as_ref().unwrap().rental_status == RentalStatus::Free)
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
    ) -> Result<Vec<u32>, ControllerError> {
        self.move_to_retrieved(slot)?;
        println!("Moved to retrieved");

        self.rent_box(user)?;
        println!("Rented box");
        self.db.new_rental(user, slot).await?;
        println!("Updated database");

        Ok(self.move_into_storage()?)
    }

    // TODO: update database
    pub async fn retrieve_object(
        &mut self,
        slot: &Vec<u32>,
        user: &str,
    ) -> Result<Vec<u32>, ControllerError> {
        self.move_to_retrieved(slot)?;
        println!("Moved to retrieved");

        self.unrent_box()?;
        println!("Unrented box");
        // This should belong to the rent/unrent function
        // also this might cause a desync between the database and the tower if the above succeeds but this fails
        self.db.finish_rental(user, slot).await?;
        println!("Updated database");

        Ok(self.move_into_storage()?)
    }

    fn move_to_retrieved(&mut self, slot: &Vec<u32>) -> Result<(), ControllerError> {
        let slot = self
            .slots
            .get_mut(slot)
            .ok_or(ControllerError::SlotNotFound)?
            .take()
            .ok_or(ControllerError::NoBoxAtLocation)?;
        self.retrieved_slot = Some(slot);

        Ok(())
    }

    fn move_into_storage(&mut self) -> Result<Vec<u32>, ControllerError> {
        let slot = self
            .retrieved_slot
            .take()
            .ok_or(ControllerError::NoBoxRetrieved)?;

        let slot_location = self.find_free_slot()?;

        self.slots
            .get_mut(&slot_location)
            .ok_or(ControllerError::SlotNotFound)?
            .replace(slot);

        Ok(slot_location)
    }

    // TODO: fix this
    pub fn slot_exists(&self, slot: &Vec<u32>) -> Result<bool, ControllerError> {
        let a = self
            .slots
            .get(slot)
            .ok_or(ControllerError::SlotNotFound)?
            .as_ref()
            .ok_or(ControllerError::NoBoxAtLocation)?;

        Ok(true)
    }

    pub fn slot_rented_by_user(
        &self,
        slot: &Vec<u32>,
        user_id: &str,
    ) -> Result<bool, ControllerError> {
        let slot = self
            .slots
            .get(slot)
            .ok_or(ControllerError::SlotNotFound)?
            .as_ref()
            .ok_or(ControllerError::NoBoxAtLocation)?;

        Ok(slot.rental_status == RentalStatus::Rented(user_id.to_string()))
    }

    // TODO: update database
    pub fn rent_box(&mut self, user_id: &str) -> Result<(), ControllerError> {
        let mut slot = self
            .retrieved_slot
            .take()
            .ok_or(ControllerError::NoBoxRetrieved)?;

        if slot.rental_status != RentalStatus::Free {
            return Err(ControllerError::BoxOccupied);
        }

        slot.rental_status = RentalStatus::Rented(user_id.to_string());

        self.retrieved_slot = Some(slot);

        Ok(())
    }

    // TODO: update database
    pub fn unrent_box(&mut self) -> Result<(), ControllerError> {
        let mut slot = self
            .retrieved_slot
            .take()
            .ok_or(ControllerError::NoBoxRetrieved)?;

        match slot.rental_status {
            RentalStatus::Free => return Err(ControllerError::BoxOccupied),
            RentalStatus::Rented(_) => {
                slot.rental_status = RentalStatus::Free;
            }
        }

        self.retrieved_slot = Some(slot);

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
