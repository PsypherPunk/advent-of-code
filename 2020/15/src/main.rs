use std::fs;

use ::day15::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let numbers = get_numbers(&input);

    println!(
        "…what will be the 2020th number spoken? {}",
        get_nth_number_for_input(&numbers, 2020),
    );
}
