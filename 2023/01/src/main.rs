use std::fs;

use ::day01::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of all of the calibration values? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the sum of all of the calibration values? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
