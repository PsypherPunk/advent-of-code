use std::fs;
use std::str::FromStr;

use ::day07::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let tower = Tower::from_str(&input).unwrap();

    println!(
        "What is the name of the bottom program? {}",
        tower.get_bottom_program(),
    );

    println!(
        "…what would its weight need to be…? {}",
        tower.get_corrected_wrong_weight(),
    );
}
