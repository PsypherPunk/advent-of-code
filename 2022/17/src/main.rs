use std::fs;

use ::day17::*;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many units tall will the tower of rocks be after 2022 rocks have stopped falling? {}",
        get_part_one(&input)?,
    );

    println!(
        "How tall will the tower be after 1000000000000 rocks have stopped? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
