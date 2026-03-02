use serde_json;
use std::{env, path};

pub mod movement; // New Movement Module
pub mod plan;
pub mod plan_domain;

use movement::movement::Movement;

use crate::plan::plan::Plan;

fn main() {
    println!("Hello, world!");
    let current_directory = env::current_dir();
    println!("Current Directory is {:?}", current_directory);

    let move_toe = std::fs::read_to_string("src/movement/YDelt.movetoe").unwrap();
    let y_delt: Movement = serde_json::from_str(&move_toe).unwrap();
    // println!("Was able to decode y_delt {:?}", y_delt);

    let plan_toe = std::fs::read_to_string("src/plan/plantoeExample.plantoe").unwrap();
    let plan_file: Plan = serde_json::from_str(&plan_toe).unwrap();
    println!("Was able to decode plantoeExample {:?}", plan_file);
}
