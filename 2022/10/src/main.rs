use std::fs;

use ::day10::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of these six signal strengths? {}",
        get_part_one(&input),
    );

    println!("{}", get_part_two(&input),);
}
