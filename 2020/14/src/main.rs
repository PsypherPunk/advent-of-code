use std::fs;

use ::day14::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of all values left in memory after it completes? {}",
        read_initialization_program(&input),
    );

    println!(
        "What is the sum of all values left in memory after it completes? {}",
        read_initialization_program_v2(&input),
    );
}
