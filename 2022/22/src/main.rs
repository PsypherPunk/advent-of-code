use std::fs;

use ::day22::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the final password? {}", get_part_one(&input));

    println!("What is the final password? {}", get_part_two(&input));
}
