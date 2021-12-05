use std::fs;

use ::day04::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What will your final score be if you choose that board? {}",
        get_part_one(&input),
    );

    println!(
        "Once it wins, what would its final score be? {}",
        get_part_two(&input),
    );
}
