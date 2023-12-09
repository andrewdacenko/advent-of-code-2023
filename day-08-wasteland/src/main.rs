use std::env;
use std::fs;

mod navigator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let steps: usize = navigator::calculate_steps(contents.as_str());
    println!("Steps:\n{steps}");
    let ghost_steps = navigator::calculate_ghost_steps(contents.as_str());
    println!("Ghost steps:\n{ghost_steps}");
}
