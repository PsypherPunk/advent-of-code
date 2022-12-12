use std::fs;

use ::day12::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the fewest steps required to move from your current position to the location that should get the best signal? {}",
        get_part_one(&input)?,
    );

    println!(
        "{}",
        get_part_two(&input),
    );

    Ok(())
}
