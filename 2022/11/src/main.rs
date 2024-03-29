use std::fs;

use ::day11::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans? {}",
        get_part_one(&input)?,
    );

    println!(
        "…what is the level of monkey business after 10000 rounds? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
