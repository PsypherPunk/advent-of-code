use std::fs;

use ::day02::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the IDs of those games? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the sum of the power of these sets? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
