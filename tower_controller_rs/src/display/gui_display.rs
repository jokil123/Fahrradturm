use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use crate::{
    storage_box::{
        box_location::BoxLocation, logistic_state::LogisticState, storage_box::StorageBox,
    },
    tower::Tower,
};

use std::{
    ops::{Deref, DerefMut},
    sync::{mpsc::Receiver, Arc, Mutex, MutexGuard},
    thread,
};

// use super::tower_display::TowerDisplay;

pub struct GUIDisplay {
    app: app::App,
    window: Arc<Mutex<Window>>,
    reciever: Arc<Mutex<Receiver<DisplayMessage>>>,
    tower: Arc<Mutex<Tower>>,
}

// useful generization once led display is implemented
// impl TowerDisplay for gui_display {}

pub enum DisplayMessage {
    Update,
}

impl GUIDisplay {
    pub fn new(reciever: Receiver<DisplayMessage>, tower: Arc<Mutex<Tower>>) -> Self {
        let app = app::App::default();
        let mut window = Window::new(100, 100, 400, 300, "Tower Controller");
        window.make_resizable(true);

        window.end();

        let mut display = GUIDisplay {
            app,
            window: Arc::new(Mutex::new(window)),
            reciever: Arc::new(Mutex::new(reciever)),
            tower: tower,
        };

        display.generate_content();

        return display;
    }

    pub fn clear_window(&mut self) {
        print!("clearing window");
        let mut window_lock = self.window.lock().unwrap();
        window_lock.clear();
    }

    pub fn generate_content(&mut self) {
        println!("generating content");
        let mut window_lock = self.window.lock().unwrap();
        let tower_lock = self.tower.lock().unwrap();

        window_lock.begin();

        let mut grid = fltk::group::HGrid::new(0, 0, 400, 300, "");
        grid.set_params(
            tower_lock.storage_layout.0 as i32,
            tower_lock.storage_layout.1 as i32,
            5,
        );

        let mut boxes = tower_lock
            .storage
            .iter()
            .collect::<Vec<(&Arc<BoxLocation>, &Option<StorageBox>)>>();

        boxes.sort_by(|(a, _), (b, _)| a.value().cmp(&b.value()));

        boxes.iter().for_each(|(location, storage_box)| {
            let mut frame = Frame::default()
                .with_size(15, 15)
                .with_label(format!("Box {}", location).as_str());

            frame.set_frame(FrameType::FlatBox);

            frame.set_color(match storage_box {
                None => Color::White,
                Some(box_type) => match box_type.logistic_state {
                    LogisticState::Stored(_) => Color::Green,
                    LogisticState::InTransit => Color::Yellow,
                    LogisticState::Retrieved => Color::Red,
                },
            });

            // grid.insert(&mut frame, location.level as usize, location.index as usize);
        });

        grid.end();

        // window_lock.add(&grid);
        window_lock.end();

        window_lock.set_damage(true);

        println!("generating content done");
    }

    pub fn run(&mut self) {
        {
            println!("aquiring window lock");
            self.window.lock().unwrap().show();
            println!("released window lock");
        }

        println!("creating proxy channel");
        let (a_s, a_r) = app::channel::<DisplayMessage>();

        {
            let reciever = self.reciever.clone();
            thread::Builder::new()
                .name("message proxy".to_string())
                .spawn(move || {
                    println!("starting message reciever thread");
                    loop {
                        println!("waiting for message");
                        match reciever.lock().unwrap().recv() {
                            Ok(msg) => a_s.send(msg),
                            Err(_) => break,
                        };
                        println!("got message");
                    }
                    println!("exiting message reciever thread");
                })
                .unwrap();
        }

        println!("running gui main loop");
        while self.app.wait() {
            if let Some(msg) = a_r.recv() {
                println!("got message");
                self.generate_content();
            }
        }
    }
}
