use std::fs;

use ::day09::{get_part_one, get_part_two};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the largest area of any rectangle you can make? {}",
        get_part_one(&input)?,
    );

    println!(
        "…what is the largest area of any rectangle you can make using only red and green tiles? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
