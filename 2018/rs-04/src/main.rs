use std::fs;

use ::day04::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the ID of the guard you chose multiplied by the minute you chose? {}",
        get_strategy_one(&input),
    );

    println!(
        "What is the ID of the guard you chose multiplied by the minute you chose? {}",
        get_strategy_two(&input),
    );
}
