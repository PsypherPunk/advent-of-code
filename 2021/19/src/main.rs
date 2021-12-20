use std::fs;

use ::day19::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("How many beacons are there? {}", get_part_one(&input));

    println!("{}", get_part_two(&input),);
}
