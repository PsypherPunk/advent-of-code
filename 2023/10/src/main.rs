use std::fs;

use ::day10::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many steps along the loop does it take…? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many tiles are enclosed by the loop? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
