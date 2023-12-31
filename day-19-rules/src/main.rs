use std::env;
use std::fs;

mod processor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = processor::sum_parts(&contents);
    println!("Sum:\n{sum}");
    let sum_ranges = processor::sum_ranges(&contents);
    println!("Sum ranges:\n{sum_ranges}");
}
