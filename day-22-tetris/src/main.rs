use std::env;
use std::fs;

mod layering;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum = layering::count_redundant(&contents);
    println!("Sum:\n{sum}");
}
