use std::fs;

use ::day11::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many total flashes are there after 100 steps? {}",
        get_part_one(&input),
    );

    println!("{}", get_part_two(&input),);
}