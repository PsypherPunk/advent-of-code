use std::fs;

use ::day16::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "In what order are the programs standing…? {}",
        perform_dance(&input),
    );
}
