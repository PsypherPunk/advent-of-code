use std::fs;

use ::day07::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What are the total winnings? {}", get_part_one(&input)?);

    println!("What are the new total winnings? {}", get_part_two(&input)?);

    Ok(())
}
