use std::sync::Arc;

use super::box_location::BoxLocation;

#[derive(Debug, PartialEq, Eq)]
pub enum LogisticState {
    Stored(Arc<BoxLocation>),
    InTransit,
    Retrieved,
}
