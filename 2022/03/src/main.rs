use std::fs;

use ::day03::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the priorities of those item types? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the sum of the priorities of those item types? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
