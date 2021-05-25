use std::fs;

use ::day07::*;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let instructions = Instructions::from_str(&input).unwrap();

    println!(
        "In what order should the steps in your instructions be completed? {}",
        instructions.get_steps_order(),
    );
}
