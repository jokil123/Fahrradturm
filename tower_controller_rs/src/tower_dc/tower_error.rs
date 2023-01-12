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
    #[error("Box not rented")]
    BoxNotRented,
    #[error("Box already rented")]
    BoxAlreadyRented,
}
