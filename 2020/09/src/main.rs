use std::fs;

use ::day09::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let numbers = get_numbers(&input);
    let invalid_number = get_first_invalid_number(&numbers, 25);

    println!(
        "What is the first number that does not have this property? {}",
        invalid_number,
    );

    println!(
        "What is the encryption weakness in your XMAS-encrypted list of numbers? {}",
        get_encryption_weakness(&numbers, invalid_number),
    );
}
