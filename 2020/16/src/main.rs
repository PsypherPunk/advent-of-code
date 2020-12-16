use std::fs;
use std::str::FromStr;

use ::day16::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let document = Document::from_str(&input).unwrap();

    println!(
        "What is your ticket scanning error rate? {}",
        document.get_ticket_scanning_error_rate(),
    );
}
