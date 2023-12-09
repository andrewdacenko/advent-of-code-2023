use std::env;
use std::fs;

mod navigator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let steps: usize = navigator::calculate_steps(contents.as_str());
    println!("Steps:\n{steps}");
    // let sum_with_joker: usize = cards::winnings(contents.as_str(), true).iter().sum();
    // println!("Sum with joker:\n{sum_with_joker}");
}
