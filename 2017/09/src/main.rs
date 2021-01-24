use std::fs;

use ::day09::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the total score…? {}",
        get_stream_for_input(&input).score,
    );

    println!(
        "How many non-canceled characters are within the garbage…? {}",
        get_stream_for_input(&input).garbage,
    );
}
