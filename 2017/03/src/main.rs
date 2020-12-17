use std::fs;

use ::day03::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let n = input.trim().parse().unwrap();

    println!("How many steps…? {}", get_manhattan_distance(n),);
}
