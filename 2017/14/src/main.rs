use std::fs;

use ::day14::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many squares are used? {}",
        get_used_square_count(&input.trim()),
    );
}
