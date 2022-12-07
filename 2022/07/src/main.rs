use std::fs;

use ::day07::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the total sizes of those directories? {}",
        get_part_one(&input),
    );

    println!("{}", get_part_two(&input),);
}
