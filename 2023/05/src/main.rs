use std::fs;

use ::day05::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the lowest location number that corresponds to any of the initial seed numbers? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the lowest location number that corresponds to any of the initial seed numbers? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
