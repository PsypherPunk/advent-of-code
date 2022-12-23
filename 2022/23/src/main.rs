use std::fs;

use ::day23::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many empty ground tiles does that rectangle contain? {}",
        get_part_one(&input)?,
    );

    println!(
        "What is the number of the first round where no Elf moves? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
