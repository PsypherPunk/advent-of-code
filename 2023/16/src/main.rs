use std::fs;

use ::day16::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many tiles end up being energized? {}",
        get_part_one(&input)?,
    );

    println!(
        "…how many tiles are energized in that configuration? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
