use std::fs;

use ::day19::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted? {}",
        get_part_one(&input)?,
    );

    println!(
        "How many distinct combinations of ratings will be accepted by the Elves' workflows? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
