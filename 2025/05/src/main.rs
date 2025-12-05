use std::fs;

use ::day05::{get_part_one, get_part_two};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many of the available ingredient IDs are fresh? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
