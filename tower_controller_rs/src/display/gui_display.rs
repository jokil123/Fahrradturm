use fltk::{
    app,
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use crate::{
    storage_box::{
        box_location::BoxLocation, box_type::BoxType, rental_status, storage_box::StorageBox,
    },
    tower::Tower,
};

use std::{
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
    Stop,
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

    fn clear_window(&mut self) {
        print!("clearing window");
        let mut window_lock = self.window.lock().unwrap();
        window_lock.clear();
    }

    fn generate_content(&mut self) {
        println!("generating content");
        let mut window_lock = self.window.lock().unwrap();
        let tower_lock = self.tower.lock().unwrap();

        window_lock.begin();

        let mut grid = fltk_grid::Grid::default_fill();
        grid.set_layout(
            tower_lock.storage_layout.0 as i32,
            tower_lock.storage_layout.1 as i32,
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

            match storage_box {
                None => frame.set_color(Color::White),
                Some(storage_box) => {
                    match storage_box.box_type {
                        BoxType::Storage => frame.set_frame(FrameType::RFlatBox),
                        BoxType::Bicycle => frame.set_frame(FrameType::FlatBox),
                    }
                    match storage_box.rental_status {
                        rental_status::RentalStatus::Available => frame.set_color(Color::Green),
                        rental_status::RentalStatus::Rented(_) => frame.set_color(Color::Blue),
                    }
                }
            }

            grid.insert(&mut frame, location.level as usize, location.index as usize);
        });

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
                match msg {
                    DisplayMessage::Update => {
                        println!("got update message");
                        self.clear_window();
                        self.generate_content();
                    }
                    DisplayMessage::Stop => {
                        println!("got stop message");
                        break;
                    }
                }
            }
        }
    }
}
