use std::fs;

use ::day15::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is its tuning frequency? {}",
        get_part_one(&input, 2_000_000),
    );

    println!(
        "{}",
        get_part_two(&input),
    );
}
