use std::env;
use std::fs;

mod mirrors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = contents
        .split("\n\n")
        .map(|line| mirrors::count_mirrors(line, &false))
        .sum();
    println!("Sum:\n{sum}");
    let sum_with_smudge: usize = contents
        .split("\n\n")
        .map(|line| mirrors::count_mirrors(line, &true))
        .sum();
    println!("Sum with smudge:\n{sum_with_smudge}");
}
