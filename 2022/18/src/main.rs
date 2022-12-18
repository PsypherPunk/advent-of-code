use std::fs;

use ::day18::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the surface area of your scanned lava droplet? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the exterior surface area of your scanned lava droplet? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
