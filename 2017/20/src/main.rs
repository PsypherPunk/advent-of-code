use std::fs;
use std::str::FromStr;

use ::day20::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut buffer = Buffer::from_str(&input).unwrap();

    println!(
        "Which particle will stay closest to position <0,0,0> …? {}",
        buffer.get_closest_to_zero(),
    );
}
