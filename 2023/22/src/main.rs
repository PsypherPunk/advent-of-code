use std::fs;

use ::day22::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many bricks could be safely chosen as the one to get disintegrated? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the sum of the number of other bricks that would fall? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
