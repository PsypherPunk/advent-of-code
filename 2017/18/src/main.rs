use std::fs;

use ::day18::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the value of the recovered frequency…? {}",
        do_duet(&input),
    );
}
