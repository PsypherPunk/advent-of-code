use std::fs;

use ::day24::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the largest model number accepted by MONAD? {}",
        get_part_one(&input),
    );

    println!(
        "What is the smallest model number accepted by MONAD?{}",
        get_part_two(&input),
    );
}
