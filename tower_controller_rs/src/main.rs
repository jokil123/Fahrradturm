use core::time;
use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use tower_controller_rs::{
    display::gui_display::{DisplayMessage, GUIDisplay},
    storage_box::StorageBox,
    tower::Tower,
};

fn main() {
    println!("running test");
    let mut tower = Arc::new(Mutex::new(Tower::new(5, 5)));

    let (sender, reciever) = channel::<DisplayMessage>();

    let handle = {
        thread::spawn(|| {
            println!("Starting GUI thread");
            let mut display = GUIDisplay::new(reciever, tower);
            display.run();
        })
    };

    thread::sleep(time::Duration::from_secs(1));

    {
        let tower_lock = tower.lock().unwrap();
        let empty_space = tower_lock.find_available_storage();

        tower_lock.storage.insert(
            empty_space,
            Some(StorageBox {
                box_type: BoxType::Bicylcle,
                rental_status: RentalStatus::Available,
                logistic_state: LogisticState::Stored(empty_space),
            }),
        );
    }

    sender.send(DisplayMessage::Update).unwrap();

    handle.join().unwrap();
}
