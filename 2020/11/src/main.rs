use std::fs;
use std::str::FromStr;

use ::day11::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let seat_layout = SeatLayout::from_str(&input).unwrap();

    println!(
        "How many seats end up occupied? {}",
        get_stable_layout(&seat_layout).get_occupied_seat_count(),
    );
}
