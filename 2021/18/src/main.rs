use std::fs;

use ::day18::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the magnitude of the final sum? {}",
        get_part_one(&input),
    );

    println!("{}", get_part_two(&input),);
}
