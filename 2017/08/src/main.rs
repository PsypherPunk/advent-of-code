use std::fs;

use ::day08::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let registers = get_registers(&input);

    println!(
        "What is the largest value in any register…? {}",
        get_highest_value(&registers),
    );
}
