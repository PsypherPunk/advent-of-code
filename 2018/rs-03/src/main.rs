use std::fs;

use ::day03::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many square inches of fabric are within two or more claims? {}",
        get_overlapping_inches_count(&input),
    );

    println!(
        "What is the ID of the only claim that doesn't overlap? {}",
        get_non_overlapping_claim_id(&input),
    );
}
