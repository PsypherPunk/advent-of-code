use std::fs;

use ::day05::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "After the rearrangement procedure completes, what crate ends up on top of each stack? {}",
        get_part_one(&input)?,
    );

    println!(
        "After the rearrangement procedure completes, what crate ends up on top of each stack? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
