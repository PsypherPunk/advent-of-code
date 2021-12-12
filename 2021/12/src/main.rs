use std::fs;

use ::day12::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many paths through this cave system are there that visit small caves at most once? {}",
        get_part_one(&input),
    );

    println!("{}", get_part_two(&input),);
}
