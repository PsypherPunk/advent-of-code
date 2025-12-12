use std::fs;

use ::day12::get_part_one;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many of the regions can fit all of the presents listed? {}",
        get_part_one(&input)?,
    );

    Ok(())
}
