use std::fs;

use ::day18::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the sum of the resulting values? {}",
        get_homework_sum(&input),
    );

    println!(
        "What do you get if you add up the results…using these new rules? {}",
        get_advanced_homework_sum(&input),
    );
}
