use std::fs;

use ::day22::*;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut cave = Cave::from_str(&input).unwrap();

    println!("What is the total risk level…? {}", cave.get_risk_level());
}
