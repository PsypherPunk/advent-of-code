use std::fs;

use ::day10::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What message will eventually appear in the sky?\n\n{}\n",
        get_part_one(&input)?,
    );

    println!(
        "…exactly how many seconds would they have needed to wait for that message to appear? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
