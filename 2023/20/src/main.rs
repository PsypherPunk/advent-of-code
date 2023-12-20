use std::fs;

use ::day20::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent? {}",
        get_part_one(&input)?,
    );

    println!("{}", get_part_two(&input)?,);

    Ok(())
}
