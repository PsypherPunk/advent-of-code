use std::fs;

use ::day25::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you multiply the sizes of these two groups together? {}",
        get_part_one(&input)?,
    );

    Ok(())
}
