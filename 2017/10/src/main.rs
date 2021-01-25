use std::fs;

use ::day10::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let lengths = get_lengths(&input);

    let mut knot_hash = KnotHash::new(0, 255);
    knot_hash.apply_lengths(&lengths);
    println!(
        "…what is the result of multiplying the first two numbers in the list? {}",
        knot_hash.get_product_of_first_two_numbers(),
    );
}
