use std::fs;

use ::day14::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many units of sand come to rest before sand starts flowing into the abyss below? {}",
        get_part_one(&input),
    );

    println!(
        "How many units of sand come to rest? {}",
        get_part_two(&input),
    );
}
