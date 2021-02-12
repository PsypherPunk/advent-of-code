use std::fs;
use std::str::FromStr;

use ::day15::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut generators = Generators::from_str(&input).unwrap();

    println!(
        "After 40 million pairs, what is the judge's final count? {}",
        generators.get_judgement(40_000_000),
    );
}
