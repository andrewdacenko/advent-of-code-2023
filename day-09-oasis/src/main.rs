use std::env;
use std::fs;

mod history;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: i64 = contents
        .split("\n")
        .map(|line| history::extrapolate_next(line))
        .sum();
    println!("Sum:\n{sum}");
    let sum_previous: i64 = contents
        .split("\n")
        .map(|line| history::extrapolate_previous(line))
        .sum();
    println!("Sum previous:\n{sum_previous}");
}
