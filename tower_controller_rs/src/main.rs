use core::time;
use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use tower_controller_rs::{
    display::gui_display::{DisplayMessage, GUIDisplay},
    tower::Tower,
};

fn main() {
    let mut tower = Arc::new(Mutex::new(Tower::new(5, 5)));

    let (sender, reciever) = channel::<DisplayMessage>();

    let handle = {
        thread::spawn(|| {
            let mut display = GUIDisplay::new(reciever, tower);
            display.run();
        })
    };

    thread::sleep(time::Duration::from_secs(1));

    sender.send(DisplayMessage::Update).unwrap();

    handle.join().unwrap();
}
