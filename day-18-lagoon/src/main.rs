use std::env;
use std::fs;

mod digger;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = digger::volume(&contents);
    println!("Sum:\n{sum}");
}
