use firestore::errors::FirestoreError;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Firestore Error: {0}")]
    FirestoreError(#[from] FirestoreError),
    #[error("Tower not found")]
    TowerNotFound,
    #[error("Slot not found")]
    SlotNotFound,
    #[error("Slot empty")]
    SlotEmpty,
    #[error("No free slots")]
    NoFreeSlots,
}
