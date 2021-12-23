use std::fs;

use ::day23::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the least energy required to organize the amphipods? {}",
        get_part_one(&input),
    );

    println!(
        "…what is the least energy required to organize the amphipods? {}",
        get_part_two(&input),
    );
}
