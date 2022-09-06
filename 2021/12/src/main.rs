use std::fs;

use ::day12::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many paths through this cave system are there that visit small caves at most once? {}",
        get_part_one(&input)?,
    );

    println!(
        "…how many paths through this cave system are there? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
