use std::fs;

use ::day25::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many constellations are formed by the fixed points in spacetime? {}",
        get_constellation_count(&input),
    );
}
