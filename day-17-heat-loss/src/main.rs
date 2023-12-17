use std::env;
use std::fs;

mod router;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let sum: usize = router::min_heat_loss(&contents);
    println!("Sum:\n{sum}");
    let sum_ultra: usize = router::min_heat_loss_ultra(&contents);
    println!("Sum ultra:\n{sum_ultra}");
}
