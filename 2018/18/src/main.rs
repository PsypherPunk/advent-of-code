use std::fs;

use ::day18::*;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let area = LumberCollectionArea::from_str(&input).unwrap();

    println!(
        "What will the total resource value of the lumber collection area be after 10 minutes? {}",
        area.get_resource_value_after_minutes(10),
    );
}
