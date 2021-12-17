use std::fs;

use ::day16::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what do you get if you add up the version numbers in all packets? {}",
        get_part_one(&input),
    );
}
