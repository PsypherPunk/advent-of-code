use std::fs;

use ::day10::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the total syntax error score for those errors? {}",
        get_part_one(&input),
    );
}
