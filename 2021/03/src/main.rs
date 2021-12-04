use std::fs;

use ::day03::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the power consumption of the submarine? {}",
        get_part_one(&input),
    );
}
