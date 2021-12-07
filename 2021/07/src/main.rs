use std::fs;

use ::day07::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How much fuel must they spend to align to that position? {}",
        get_part_one(&input),
    );
}
