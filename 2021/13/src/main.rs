use std::fs;

use ::day13::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many dots are visible after completing just the first fold instruction on your transparent paper? {}",
        get_part_one(&input),
    );

    println!(
        "What code do you use to activate the infrared thermal imaging camera system?\n{}",
        get_part_two(&input),
    );
}
