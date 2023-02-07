use crate::storage_box::{box_location::BoxLocation, box_type::BoxType};

pub struct Job {
    pub created_by: String,
    pub task: Task,
}

pub enum Task {
    Store(BoxType),
    /// Retrieve a Box from the tower
    Retrieve(BoxLocation),
}
