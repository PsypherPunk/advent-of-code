use std::fs;

use ::day08::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many steps are required to reach ZZZ? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many steps does it take before you're only on nodes that end with Z? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
