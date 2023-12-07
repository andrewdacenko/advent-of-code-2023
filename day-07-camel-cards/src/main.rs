use std::env;
use std::fs;

mod cards;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = cards::winnings(contents.as_str(), false).iter().sum();
    println!("Sum:\n{sum}");
    let sum_with_joker: usize = cards::winnings(contents.as_str(), true).iter().sum();
    println!("Sum with joker:\n{sum_with_joker}");
}
