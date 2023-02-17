use std::{collections::HashMap, default, sync::Arc};

use crate::controller_error::ControllerError;

#[derive(Debug)]
pub struct Tower {
    pub id: String,
    pub slots: HashMap<Vec<u32>, Option<Slot>>,
    pub retrieved_slot: Option<Slot>,
    pub layout: Vec<u32>,
}

impl Tower {
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

    pub fn store(&mut self, slot: &Vec<u32>) -> Result<Vec<u32>, ControllerError> {
        self.move_to_retrieved(slot)?;

        Ok(self.move_into_storage()?)
    }

    pub fn retrieve(&mut self, slot: &Vec<u32>) -> Result<(), ControllerError> {
        todo!();
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
}

#[derive(Debug, PartialEq, Default)]
pub enum RentalStatus {
    #[default]
    Free,
    Rented(String),
    // Reserved(String),
}
