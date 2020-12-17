use std::fs;

use ::day02::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the checksum…? {}", get_checksum(&input));

    println!(
        "What is the sum of each row's result…? {}",
        get_evenly_divisible_values(&input),
    );
}
