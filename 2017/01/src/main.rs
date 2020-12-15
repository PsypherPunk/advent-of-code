use std::fs;

use ::day01::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the solution to your captcha? {}",
        get_sum_of_identical_digits(&input),
    );

    println!(
        "What is the solution to your new captcha? {}",
        get_sum_of_halfway_digits(&input),
    );
}
