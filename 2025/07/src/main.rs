use std::fs;

use ::day07::{get_part_one, get_part_two};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many times will the beam be split? {}",
        get_part_one(&input)?,
    );

    println!(
        "{}",
        get_part_two(&input)?,
    );

    Ok(())
}
