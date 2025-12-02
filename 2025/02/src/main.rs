use std::fs;

use ::day02::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you add up all of the invalid IDs? {}",
        get_part_one(&input)?,
    );

    println!(
        "What do you get if you add up all of the invalid IDs using these new rules? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
