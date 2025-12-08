use std::fs;

use ::day08::{get_part_one, get_part_two};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what do you get if you multiply together the sizes of the three largest circuits? {}",
        get_part_one(&input, 1000)?,
    );

    println!(
        "What do you get if you multiply together the X coordinates of the last two junction boxes you need to connect? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
