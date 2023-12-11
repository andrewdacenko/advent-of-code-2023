use std::env;
use std::fs;

mod galaxy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let distance: usize = galaxy::shortest_paths(contents.as_str()).iter().sum();
    println!("Distance:\n{distance}");
}
