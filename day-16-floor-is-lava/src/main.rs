use std::env;
use std::fs;

mod beamer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = beamer::count_energized_tiles(&contents);
    println!("Sum:\n{sum}");
    let most_energized_sum: usize = beamer::count_most_energized_tiles(&contents);
    println!("Most energized sum:\n{most_energized_sum}");
}
