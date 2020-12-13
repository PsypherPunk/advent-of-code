use std::fs;
use std::str::FromStr;

use ::day13::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let notes = Notes::from_str(&input).unwrap();

    let (bus_id, wait) = notes.get_earliest_bus_wait();
    println!(
        "What is the ID…multiplied by the number of minutes…? {}",
        bus_id * wait,
    );

    println!(
        "What is the earliest timestamp…? {}",
        notes.get_earliest_timestamp_with_offset(),
    );
}
