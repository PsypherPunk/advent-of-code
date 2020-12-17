use std::fs;

use ::day02::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the checksum…? {}", get_checksum(&input),);
}
