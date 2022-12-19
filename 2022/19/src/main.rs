use std::fs;

use ::day19::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you add up the quality level of all of the blueprints in your list? {}",
        get_part_one(&input),
    );

    println!(
        "What do you get if you multiply these numbers together? {}",
        get_part_two(&input),
    );
}
