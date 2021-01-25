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

    let lengths = get_ascii_lengths(&input);
    let mut knot_hash = KnotHash::new(0, 255);
    knot_hash.generate_sparse_hash(&lengths);
    println!(
        "…what is the Knot Hash of your puzzle input? {}",
        knot_hash.get_dense_hash(),
    );
}
