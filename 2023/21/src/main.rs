use std::fs;

use ::day21::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many garden plots could the Elf reach in exactly 64 steps? {}",
        get_part_one(&input, 64)?,
    );

    println!(
        "…how many garden plots could the Elf reach in exactly 26501365 steps? {}",
        get_part_two(&input, 26501365)?,
    );

    Ok(())
}
