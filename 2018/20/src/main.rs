use std::fs;
use std::str::FromStr;

use ::day20::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let map = Map::from_str(&input).unwrap();

    println!(
        "What is the largest number of doors you would be required to pass through to reach a room? {}",
        map.get_most_doors(),
    );

    println!(
        "How many rooms have a shortest path from your current location that pass through at least 1000 doors? {}",
        map.get_rooms_over_1000_doors_away(),
    );
}
