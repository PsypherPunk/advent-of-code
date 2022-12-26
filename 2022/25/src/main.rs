use std::fs;

use ::day25::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What SNAFU number do you supply to Bob's console? {}",
        get_part_one(&input),
    );
}
