use std::fs;

use ::day09::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many positions does the tail of the rope visit at least once? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many positions does the tail of the rope visit at least once? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
