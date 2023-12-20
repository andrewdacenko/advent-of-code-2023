use std::env;
use std::fs;

mod pulser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum = pulser::count_pulses(&contents, 1000);
    println!("Sum:\n{sum}");
    let rx = pulser::count_pulses_till_machine_starts(&contents);
    println!("Rx: {rx}");
}
