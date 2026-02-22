use serde_json;
use std::{env, path};

pub mod decode;
pub mod movement;
pub mod plan;

use decode::decode_movetoe::MovementDecode;

fn main() {
    println!("Hello, world!");
    let current_directory = env::current_dir();
    println!("Current Directory is {:?}", current_directory);

    let move_toe = std::fs::read_to_string("src/decode/YDelt.movetoe").unwrap();
    let y_delt: MovementDecode = serde_json::from_str(&move_toe).unwrap();
    println!("Was able to decode y_delt {:?}", y_delt);
}
