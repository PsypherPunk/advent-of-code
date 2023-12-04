use std::fs;

use ::day04::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many points are they worth in total? {}",
        get_part_one(&input)?,
    );

    println!("{}", get_part_two(&input)?,);

    Ok(())
}
