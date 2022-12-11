use std::fs;

use ::day11::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans? {}",
        get_part_one(&input),
    );

    println!("{}", get_part_two(&input),);
}
