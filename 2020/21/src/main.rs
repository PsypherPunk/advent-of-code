use std::fs;
use std::str::FromStr;

use ::day21::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let food = Food::from_str(&input).unwrap();

    println!(
        "How many times do any of those ingredients appear? {}",
        food.get_safe_count(),
    );
}
