use std::fs;

use ::day10::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of these six signal strengths? {}",
        get_part_one(&input)?,
    );

    println!(
        "What eight capital letters appear on your CRT?\n\n{}",
        get_part_two(&input)?,
    );

    Ok(())
}
