use std::fs;

use ::day20::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the three numbers that form the grove coordinates? {}",
        get_part_one(&input)?,
    );

    println!(
        "{}",
        get_part_two(&input),
    );

    Ok(())
}
