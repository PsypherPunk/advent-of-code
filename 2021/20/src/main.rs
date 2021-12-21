use std::fs;

use ::day20::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many pixels are lit in the resulting image? {}",
        get_part_one(&input),
    );

    println!(
        "How many pixels are lit in the resulting image? {}",
        get_part_two(&input),
    );
}
