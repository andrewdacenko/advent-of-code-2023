use calibrate;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: u32 = contents
        .split("\n")
        .map(|line| calibrate::calibrate(line))
        .sum();
    println!("Sum:\n{sum}");
}
