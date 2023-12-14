use std::env;
use std::fs;

mod reflector;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = reflector::count_load_north(&contents);
    println!("Sum:\n{sum}");
}
