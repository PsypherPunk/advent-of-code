use std::fs;

use ::day15::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the lowest total risk of any path from the top left to the bottom right? {}",
        get_part_one(&input),
    );

    println!(
        "…what is the lowest total risk of any path from the top left to the bottom right? {}",
        get_part_two(&input),
    );
}
