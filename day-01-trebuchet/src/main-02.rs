use crate::calibrate::digits_and_text::digits_and_text;
use std::env;
use std::fs;

pub mod calibrate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: u32 = contents.split("\n").map(|line| digits_and_text(line)).sum();
    println!("Sum:\n{sum}");
}
