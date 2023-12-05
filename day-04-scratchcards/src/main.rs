mod lottery;

use std::env;
use std::fs;

use crate::lottery::card_value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: u32 = contents.split("\n").map(|line| card_value(line)).sum();
    println!("Sum:\n{sum}");
}
