use std::env;
use std::fs;

mod lever;
mod reflector;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = reflector::count_load_north(&contents);
    println!("Sum:\n{sum}");
    let sum_after_cycles: usize = lever::count_load_north_after_cycles(&contents, 1_000_000_000);
    println!("Sum after cycles:\n{sum_after_cycles}");
}
