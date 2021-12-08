use std::fs;

use ::day08::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "In the output values, how many times do digits 1, 4, 7, or 8 appear? {}",
        get_part_one(&input),
    );
}
