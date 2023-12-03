use std::env;
use std::fs;

use crate::games::game_power;
use crate::games::process_game;

mod games;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = contents.split("\n").map(|line| process_game(line)).sum();
    println!("Sum:\n{sum}");
    let sum_power: usize = contents.split("\n").map(|line| game_power(line)).sum();
    println!("Sum power:\n{sum_power}");
}
