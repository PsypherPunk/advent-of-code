use std::fs;

use ::day08::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many trees are visible from outside the grid? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the highest scenic score possible for any tree? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
