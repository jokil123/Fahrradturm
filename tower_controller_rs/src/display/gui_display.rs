use fltk::{
    app,
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
    sync::{mpsc::Receiver, Arc, Mutex},
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
        let window = Window::new(100, 100, 400, 300, "Tower Controller");
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

    pub fn generate_content(&mut self) {
        println!("generating content");
        // window.clear();
        let tower_lock = self.tower.lock().unwrap();

        self.window
            .lock()
            .unwrap()
            .add(&Button::new(0, 0, 100, 100, "Hello World!"));

        println!("generating content done");

        // let window = Window::new(100, 100, 400, 300, "Tower Controller");

        // let mut grid = Grid::default_fill();

        // // grid.set_layout(tower.storage_layout.0 - 1, tower.storage_layout.0 - 1);
        // grid.set_layout(5, 5);
        // // grid.debug(true);

        // tower.storage.iter().for_each(|(location, storage_box)| {
        //     let mut frame = Frame::default()
        //         .with_size(15, 15)
        //         .with_label(format!("Box {}", location).as_str());

        //     frame.set_frame(FrameType::FlatBox);

        //     frame.set_color(match storage_box {
        //         None => Color::White,
        //         Some(box_type) => match box_type.logistic_state {
        //             storage_box::LogisticState::Stored(_) => Color::Green,
        //             storage_box::LogisticState::InTransit => Color::Yellow,
        //             storage_box::LogisticState::Retrieved => Color::Red,
        //         },
        //     });

        //     grid.insert(&mut frame, location.level as usize, location.index as usize);
        // });

        // window.end();

        // TODO: make sure the grid is added to the window
        // self.window.add(&grid);
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
            thread::spawn(move || {
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
            });
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
