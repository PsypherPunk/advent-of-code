use std::fs;

use ::day01::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many total Calories is that Elf carrying? {}",
        get_part_one(&input),
    );

    println!(
        "How many Calories are those Elves carrying in total? {}",
        get_part_two(&input),
    );
}
