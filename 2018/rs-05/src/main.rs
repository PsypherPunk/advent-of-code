use std::fs;

use ::day05::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many units remain after fully reacting the polymer you scanned? {}",
        get_part_one(&input),
    );

    println!(
        "What is the length of the shortest polymer you can produce {}",
        get_part_two(&input),
    );
}
