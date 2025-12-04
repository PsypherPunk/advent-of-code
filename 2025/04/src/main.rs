use std::fs;

use ::day04::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many rolls of paper can be accessed by a forklift? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many rolls of paper in total can be removed by the Elves and their forklifts? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
