use std::fs;

use ::day09::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of these extrapolated values? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the sum of these extrapolated values? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
