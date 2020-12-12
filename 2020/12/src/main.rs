use std::fs;
use std::str::FromStr;

use ::day12::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut ferry = Ferry::from_str(&input).unwrap();
    ferry.navigate();

    println!(
        "What is the Manhattan distance between that location and the ship's starting position? {}",
        ferry.get_manhattan_distanct(),
    );
}
