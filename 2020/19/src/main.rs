use std::fs;
use std::str::FromStr;

use ::day19::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let satellite = SatelliteConnection::from_str(&input).unwrap();

    println!(
        "How many messages completely match rule 0? {}",
        satellite.get_valid_message_count(),
    );

    let part_two_input = input
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");

    let satellite = SatelliteConnection::from_str(&part_two_input).unwrap();

    println!(
        "After updating rules 8 and 11, how many messages completely match rule 0? {}",
        satellite.get_valid_message_count(),
    );
}
