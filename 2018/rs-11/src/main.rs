use std::fs;

use ::day11::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the X,Y,size identifier of the square with the largest total power? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
