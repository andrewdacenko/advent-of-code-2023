use std::env;
use std::fs;

mod springs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = contents
        .split("\n")
        .map(|line| springs::arrangements(line))
        .sum();
    println!("Sum:\n{sum}");
    let sum_long: usize = contents
        .split("\n")
        .map(|line| springs::arrangements_long(line))
        .sum();
    println!("Sum long:\n{sum_long}");
}
