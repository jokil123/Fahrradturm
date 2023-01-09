use super::{box_type::BoxType, logistic_state::LogisticState, rental_status::RentalStatus};

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
