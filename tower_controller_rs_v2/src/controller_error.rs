use firestore::errors::FirestoreError;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Firestore Error: {0}")]
    FirestoreError(#[from] FirestoreError),
    #[error("Tower not found")]
    TowerNotFound,
    #[error("Slot not found")]
    SlotNotFound,
    #[error("No free slots")]
    NoFreeSlots,
    #[error("User not found")]
    UserNotFound,
    #[error("No box at location")]
    NoBoxAtLocation,
    #[error("No box retrieved")]
    NoBoxRetrieved,
    #[error("Box occupied")]
    BoxOccupied,
    #[error("Parse error")]
    ParseError,
    #[error("Box not rented")]
    BoxNotRented,
    #[error("Box not rented by user")]
    BoxNotRentedByUser,
    #[error("Invalid rental")]
    InvalidRental,
}
