use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum BoxType {
    Bicycle,
    Storage,
}
