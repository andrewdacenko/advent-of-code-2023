use std::env;
use std::fs;

use crate::engine::get_part_numbers;

mod engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = get_part_numbers(contents.as_str()).iter().sum();
    println!("Sum:\n{sum}");
}
