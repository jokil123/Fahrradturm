use crate::tower_dc::tower_error::TowerError;

use super::{box_type::BoxType, logistic_state::LogisticState, rental_status::RentalStatus};

pub struct StorageBox {
    pub box_type: BoxType,
    pub rental_status: RentalStatus,
    pub logistic_state: LogisticState,
}

impl StorageBox {
    pub fn rent(&mut self, user: String) -> Result<(), TowerError> {
        if self.rental_status != RentalStatus::Available {
            return Err(TowerError::BoxAlreadyRented);
        }

        self.rental_status = RentalStatus::Rented(user);

        Ok(())
    }

    pub fn return_box(&mut self) -> Result<(), TowerError> {
        if self.rental_status == RentalStatus::Available {
            return Err(TowerError::BoxNotRented);
        }

        self.rental_status = RentalStatus::Available;

        Ok(())
    }
}
