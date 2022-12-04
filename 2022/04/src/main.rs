use std::fs;

use ::day04::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "In how many assignment pairs does one range fully contain the other? {}",
        get_part_one(&input),
    );

    println!(
        "In how many assignment pairs do the ranges overlap? {}",
        get_part_two(&input),
    );
}
