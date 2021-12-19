use std::fs;

use ::day17::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the highest y position it reaches on this trajectory? {}",
        get_part_one(&input),
    );

    println!(
        "How many distinct initial velocity values cause the probe to be within the target area after any step? {}",
        get_part_two(&input),
    );
}
