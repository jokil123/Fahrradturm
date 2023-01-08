use std::fmt::{Display, Formatter};

pub struct StorageBox {
    pub box_type: BoxType,
    pub rental_status: RentalStatus,
    pub logistic_state: LogisticState,
}

impl StorageBox {
    fn rent() {
        todo!("implement rent")
    }

    fn return_box() {
        todo!("implement return_box")
    }
}

pub enum LogisticState {
    Stored(BoxLocation),
    InTransit,
    Retrieved,
}

pub enum RentalStatus {
    Available,
    Rented(i32),
}

pub enum BoxType {
    Bicylcle,
    Storage,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BoxLocation {
    pub level: i32,
    pub index: i32,
}

impl Display for BoxLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "l{}i{}", self.level, self.index)
    }
}
