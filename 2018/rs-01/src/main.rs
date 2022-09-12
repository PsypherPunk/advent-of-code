use std::fs;

use ::day01::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the resulting frequency…? {}",
        get_resulting_frequency(&input),
    );

    println!(
        "What is the first frequency your device reaches twice? {}",
        get_first_duplicated_frequency(&input),
    );
}
