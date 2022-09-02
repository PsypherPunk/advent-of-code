use std::fs;

use ::day10::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the total syntax error score for those errors? {}",
        get_part_one(&input)?,
    );

    println!("What is the middle score? {}", get_part_two(&input)?);

    Ok(())
}
