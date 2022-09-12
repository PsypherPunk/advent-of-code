use std::fs;

use ::day09::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the winning Elf's score? {}", get_part_one(&input)?);

    println!(
        "What would the new winning Elf's score be if the number of the last marble were 100 times larger? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
