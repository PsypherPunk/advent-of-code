use std::fs;

use ::day15::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the sum of the results? {}", get_part_one(&input)?,);

    println!(
        "What is the focusing power of the resulting lens configuration? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
