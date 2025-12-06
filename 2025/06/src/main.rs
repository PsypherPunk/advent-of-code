use std::fs;

use ::day06::{get_part_one, get_part_two};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the grand total found by adding together all of the answers to the individual problems? {}",
        get_part_one(&input)?,
    );

    println!(
        "{}",
        get_part_two(&input)?,
    );

    Ok(())
}
