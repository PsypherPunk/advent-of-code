use std::fs;

use ::day24::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many of these intersections occur within the test area? {}",
        get_part_one(&input, 200_000_000_000_000.0, 400_000_000_000_000.0)?,
    );

    println!("{}", get_part_two(&input)?,);

    Ok(())
}
