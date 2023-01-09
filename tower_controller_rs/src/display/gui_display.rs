use fltk::{
    app::{self, Receiver, Sender},
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};
use fltk_grid::Grid;

use crate::{storage_box, tower::Tower};

use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, RwLock},
    thread,
};

// use super::tower_display::TowerDisplay;

pub struct GUIDisplay {
    app: app::App,
    window: Window,
    receiver: Receiver<DisplayMessage>,
    tower: Arc<Mutex<Tower>>,
}

// useful generization once led display is implemented
// impl TowerDisplay for gui_display {}

pub enum DisplayMessage {
    Update,
}

impl GUIDisplay {
    pub fn new(tower: Arc<Mutex<Tower>>) -> (Self, Sender<DisplayMessage>) {
        let app = app::App::default();
        let mut window = Window::new(100, 100, 400, 300, "Tower Controller");

        GUIDisplay::generate_content(&mut window, tower.lock().unwrap().deref());

        window.end();

        let (sender, receiver) = app::channel::<DisplayMessage>();

        (
            GUIDisplay {
                app,
                window: window,
                receiver,
                tower: tower,
            },
            sender,
        )
    }

    pub fn generate_content(window: &mut Window, tower: &Tower) {
        window.clear();

        let mut grid = Grid::default_fill();

        grid.set_layout(tower.storage_layout.0 - 1, tower.storage_layout.0 - 1);

        tower.storage.iter().for_each(|(location, storage_box)| {
            let mut frame = Frame::default()
                .with_size(15, 15)
                .with_label(format!("Box {}", location).as_str());

            frame.set_frame(FrameType::FlatBox);

            frame.set_color(match storage_box {
                None => Color::White,
                Some(box_type) => match box_type.logistic_state {
                    storage_box::LogisticState::Stored(_) => Color::Green,
                    storage_box::LogisticState::InTransit => Color::Yellow,
                    storage_box::LogisticState::Retrieved => Color::Red,
                },
            });

            grid.insert(&mut frame, location.level as usize, location.index as usize);
        });

        window.end();

        // TODO: make sure the grid is added to the window
        // self.window.add(&grid);
    }

    pub fn run(&mut self) {
        // {
        //     let reciever = self.reciever.clone();
        //     let window = self.window.clone();
        //     let tower = self.tower.clone();

        //     thread::Builder::new()
        //         .name("Display Message Handler".to_string())
        //         .spawn(move || {
        //             // TODO: potentially handle different message types
        //             let msg = reciever.lock().unwrap().recv().unwrap();

        //             let mut window_lock = window.lock().unwrap();
        //             let tower_lock = tower.lock().unwrap();
        //             GUIDisplay::generate_content(window_lock.deref_mut(), tower_lock.deref());
        //         })
        //         .unwrap()
        // };

        // self.app.run().unwrap();

        self.window.show();

        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                let tower_lock = self.tower.lock().unwrap();
                GUIDisplay::generate_content(&mut self.window, tower_lock.deref());
            }
        }
    }
}
