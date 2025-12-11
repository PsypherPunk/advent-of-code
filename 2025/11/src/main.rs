use std::fs;

use ::day11::{get_part_one, get_part_two};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many different paths lead from you to out? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many of those paths visit both dac and fft? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
