mod fertilizer;

use std::env;
use std::fs;

use crate::fertilizer::plant_location;
use crate::fertilizer::plant_ranged_location;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let location: u64 = plant_location(contents.as_str());
    println!("Plant location:\n{location}");
    let ranged_location: u64 = plant_ranged_location(contents.as_str());
    println!("Plant ranged location:\n{ranged_location}");
}
