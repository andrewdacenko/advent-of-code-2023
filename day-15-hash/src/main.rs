use std::env;
use std::fs;

mod hash;
mod labeler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = hash::hash_string(&contents);
    println!("Sum:\n{sum}");
    let focusing_power = labeler::focusing_power(&contents);
    println!("Focusing power:\n{focusing_power}");
}
