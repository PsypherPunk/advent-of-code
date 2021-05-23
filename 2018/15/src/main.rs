use std::fs;
use std::str::FromStr;

use ::day15::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut cave = Cave::from_str(&input).unwrap();

    println!("What is the outcome…? {}", cave.get_score(),);
}
