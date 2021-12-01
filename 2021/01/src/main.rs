use std::fs;

use ::day01::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many measurements are larger than the previous measurement? {}",
        get_larger_measurements(&input),
    );

    println!(
        "How many sums are larger than the previous sum? {}",
        get_larger_sums(&input),
    );
}
