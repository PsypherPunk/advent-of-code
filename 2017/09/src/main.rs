use std::fs;

use ::day09::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the total score…? {}", get_score_for_input(&input));
}
