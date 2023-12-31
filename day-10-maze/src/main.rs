use std::env;
use std::fs;

mod maze;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let distance: usize = maze::longest_path(contents.as_str());
    println!("Distance:\n{distance}");
    let sum: usize = maze::enclosed_tiles(contents.as_str());
    println!("Enclosed:\n{sum}");
}
