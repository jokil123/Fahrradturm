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

    let handle = {
        thread::spawn(move || {
            let (mut display, s) = GUIDisplay::new(tower);
            display.run();
        })
    };

    thread::sleep(time::Duration::from_secs(1));

    // s.send(DisplayMessage::Update);

    handle.join().unwrap();
}
