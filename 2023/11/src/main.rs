use std::fs;

use ::day11::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of these lengths? {}",
        get_part_one(&input)?,
    );

    println!("{}", get_part_two(&input)?,);

    Ok(())
}
