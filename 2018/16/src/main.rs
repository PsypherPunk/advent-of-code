use std::fs;
use std::str::FromStr;

use ::day16::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let input_file = InputFile::from_str(&input).unwrap();

    println!(
        "…how many samples in your puzzle input behave like three or more opcodes? {}",
        input_file.get_multiple_match_count(),
    );
}
