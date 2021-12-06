use std::fs;

use ::day06::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many lanternfish would there be after 80 days? {}",
        get_part_one(&input),
    );
}
