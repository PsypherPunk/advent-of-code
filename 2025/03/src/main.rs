use std::fs;

use ::day03::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the total output joltage? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the new total output joltage? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
