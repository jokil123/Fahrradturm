use std::process::Command;

use crate::tower::{RentalStatus, Tower};

pub struct TowerDisplay;

const DISPLAY_SCRIPT: &str = "../../hardware_interface/set_led.py";

impl TowerDisplay {
    pub fn go(tower: &Tower) {
        let led_count = tower.slots.len();

        let mut led_string = String::new();

        for slot in tower.slots.values() {
            match slot.rental_status {
                RentalStatus::Free => led_string += &Color(0, 0, 255).to_string(),
                RentalStatus::Rented(_) => led_string += &Color(0, 255, 0).to_string(),
            }
        }

        match Command::new("sudo")
            .arg("-E")
            .arg("python")
            .arg(DISPLAY_SCRIPT)
            .arg(led_count.to_string())
            .arg(led_string)
            .spawn()
        {
            Ok(_) => println!("Displaying"),
            Err(e) => println!(
                "Command failed to start, this might be due to not running on linux: ({})",
                e
            ),
        }
    }
}

struct Color(u8, u8, u8);

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}
