use std::fs;

use ::day21::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what do you get if you multiply the score of the losing player by the number of times the die was rolled during the game? {}",
        get_part_one(&input),
    );

    println!(
        "…in how many universes does that player win? {}",
        get_part_two(&input),
    );
}
