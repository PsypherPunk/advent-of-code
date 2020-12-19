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
}
