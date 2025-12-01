use std::fs;

use ::day01::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What's the actual password to open the door? {}",
        get_part_one(&input)?,
    );

    println!(
        "…what is the password to open the door? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
