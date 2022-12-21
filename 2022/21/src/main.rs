use std::fs;

use ::day21::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What number will the monkey named root yell? {}",
        get_part_one(&input),
    );

    println!(
        "What number do you yell to pass root's equality test? {}",
        get_part_two(&input),
    );
}
