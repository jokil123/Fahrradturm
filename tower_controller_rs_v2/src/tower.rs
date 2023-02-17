use std::{collections::HashMap, sync::Arc};

use crate::controller_error::ControllerError;

#[derive(Debug)]
pub struct Tower {
    pub id: String,
    pub slots: HashMap<Vec<u32>, Option<Arc<Slot>>>,
    pub retrieved_slot: Option<Slot>,
    pub layout: Vec<u32>,
}

impl Tower {
    pub fn find_free_slot(&self) -> Result<Arc<Slot>, ControllerError> {
        todo!();
    }

    pub fn reserve_slot(&mut self) -> Result<(), ()> {
        todo!();
    }

    pub fn store(&mut self, slot: Arc<Slot>) -> Result<(), ()> {
        todo!();
    }

    pub fn retrieve(&mut self, slot: Arc<Slot>) -> Result<(), ()> {
        todo!();
    }

    pub fn get_slot(&self, slot: Vec<u32>) -> Result<Arc<Slot>, ControllerError> {
        let slot = self
            .slots
            .get(&slot)
            .ok_or(ControllerError::SlotNotFound)?
            .clone()
            .ok_or(ControllerError::SlotEmpty)?;

        Ok(slot)
    }
}

#[derive(Debug)]
pub struct Slot {
    pub rental_status: RentalStatus,
}

#[derive(Debug, PartialEq)]
pub enum RentalStatus {
    Free,
    Rented(String),
    // Reserved(String),
}
