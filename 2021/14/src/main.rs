use std::fs;

use ::day14::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you take the quantity of the most common element and subtract the quantity of the least common element? {}",
        get_part_one(&input),
    );

    println!(
        "What do you get if you take the quantity of the most common element and subtract the quantity of the least common element? {}",
        get_part_two(&input),
    );
}
