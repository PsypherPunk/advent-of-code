use std::fs;
use std::str::FromStr;

use ::day23::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut crab_cups = CrabCups::from_str(&input).unwrap();

    for _ in 0..100 {
        crab_cups.do_move();
    }

    println!(
        "What are the labels on the cups after cup 1? {}",
        crab_cups.get_labels(),
    );
}
