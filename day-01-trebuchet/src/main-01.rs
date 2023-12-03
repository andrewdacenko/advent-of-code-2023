use crate::calibrate::digits_only::digits_only;
use std::env;
use std::fs;

pub mod calibrate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: u32 = contents.split("\n").map(|line| digits_only(line)).sum();
    println!("Sum:\n{sum}");
}
