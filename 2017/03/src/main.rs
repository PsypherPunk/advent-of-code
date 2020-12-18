use std::fs;

use ::day03::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let n = input.trim().parse().unwrap();

    println!("How many steps…? {}", get_manhattan_distance(n));

    println!(
        "What is the first value written that is larger than your puzzle input? {}",
        get_first_square_spiral_sum_greater_than(n),
    );
}
