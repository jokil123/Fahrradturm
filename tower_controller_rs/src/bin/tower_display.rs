use core::time;
use std::{
    borrow::BorrowMut,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use tower_controller_rs::{
    display::gui_display::{DisplayMessage, GUIDisplay},
    storage_box::{
        box_location::BoxLocation, box_type::BoxType, logistic_state::LogisticState,
        rental_status::RentalStatus, storage_box::StorageBox,
    },
    tower::Tower,
};

use clone_all::clone_all;

fn main() {
    println!("running test");
    let mut tower = Arc::new(Mutex::new(Tower::new(5, 5)));

    let (sender, reciever) = channel::<DisplayMessage>();

    let handle = {
        clone_all!(tower);
        thread::Builder::new()
            .name("gui thread".to_string())
            .spawn(|| {
                println!("Starting GUI thread");
                let mut display = GUIDisplay::new(reciever, tower);
                display.run();
            })
            .unwrap()
    };

    thread::sleep(time::Duration::from_secs(2));

    {
        let mut tower_lock = tower.lock().unwrap();
        let empty_space = Arc::new(BoxLocation { level: 0, index: 0 });

        tower_lock.storage.insert(
            empty_space.clone(),
            Some(StorageBox {
                box_type: BoxType::Bicylcle,
                rental_status: RentalStatus::Available,
                logistic_state: LogisticState::Stored(empty_space),
            }),
        );
    }

    sender.send(DisplayMessage::Update).unwrap();

    thread::sleep(time::Duration::from_secs(2));

    {
        let mut tower_lock = tower.lock().unwrap();
        let av_box = tower_lock.find_available_box(None).unwrap();

        tower_lock.retrieve_box(av_box).unwrap();
    }

    sender.send(DisplayMessage::Update).unwrap();

    handle.join().unwrap();
}
