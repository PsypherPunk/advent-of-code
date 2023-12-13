use std::fs;

use ::day13::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What number do you get after summarizing all of your notes? {}",
        get_part_one(&input)?,
    );

    println!(
        "What number do you get after summarizing the new reflection line in each pattern in your notes? {}",
        get_part_two(&input)?,
    );

    Ok(())
}
