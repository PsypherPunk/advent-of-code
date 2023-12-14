use std::fs;

use ::day14::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the total load on the north support beams? {}",
        get_part_one(&input)?,
    );

    println!(
        "{}",
        get_part_two(&input)?,
    );

    Ok(())
}
