use std::fs;

use ::day07::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the total sizes of those directories? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the total size of that directory? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
