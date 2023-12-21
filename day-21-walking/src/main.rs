use std::env;
use std::fs;

mod walker;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let steps = args[2].parse::<usize>().unwrap();

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum = walker::count_tiles(&contents, 64);
    println!("Sum:\n{sum}");
    let sum_infinite = walker::count_tiles_infinite(&contents, steps);
    println!("Sum infinite:\n{sum_infinite}");
}
