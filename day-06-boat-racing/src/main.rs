use std::env;
use std::fs;

use crate::race::num_ways_to_win;

mod race;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = num_ways_to_win(&contents);
    println!("Sum:\n{sum}");
}
