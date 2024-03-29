use std::fs;

use ::day02::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the checksum…? {}", get_checksum(&input),);

    println! {
        "What letters are common between the two correct box IDs? {}",
        get_common_letters(&input),
    };
}
