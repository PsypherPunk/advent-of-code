use std::fs;

use ::day13::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the indices of those pairs? {}",
        get_part_one(&input),
    );

    println!(
        "What is the decoder key for the distress signal? {}",
        get_part_two(&input),
    );
}
