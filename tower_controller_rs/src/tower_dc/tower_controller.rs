use std::sync::{Arc, Mutex};

use super::tower_data::TowerData;

struct TowerController {
    tower: Arc<Mutex<TowerData>>,
}

impl TowerController {}
