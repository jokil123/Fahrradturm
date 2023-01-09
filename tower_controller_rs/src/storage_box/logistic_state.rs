use std::sync::Arc;

use super::box_location::BoxLocation;

pub enum LogisticState {
    Stored(Arc<BoxLocation>),
    InTransit,
    Retrieved,
}
