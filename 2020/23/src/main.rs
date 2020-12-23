use std::fs;

use ::day23::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut crab_cups = CrabCups::from_str(&input, input.trim().len());
    crab_cups.make_moves(100);
    println!(
        "What are the labels on the cups after cup 1? {}",
        crab_cups.get_labels(),
    );

    let mut crab_cups = CrabCups::from_str(&input, 1_000_000);
    crab_cups.make_moves(10_000_000);
    println!(
        "What do you get if you multiply their labels together? {}",
        crab_cups.get_cup_product_two_clockwise_of_one(),
    );
}
