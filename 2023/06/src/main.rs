use std::fs;

use ::day06::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you multiply these numbers together? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many ways can you beat the record in this one much longer race? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
