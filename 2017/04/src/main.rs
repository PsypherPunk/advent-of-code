use std::fs;

use ::day04::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many passphrases are valid? {}",
        get_valid_passphrase_count(&input),
    );
}
