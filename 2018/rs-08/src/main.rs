use std::fs;

use ::day08::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("{}", get_part_one(&input),);

    println!("{}", get_part_two(&input),);
}
