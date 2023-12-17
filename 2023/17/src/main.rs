use std::fs;

use ::day17::*;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the least heat loss it can incur? {}",
        get_part_one(&input)?,
    );

    println!("{}", get_part_two(&input)?,);

    Ok(())
}
