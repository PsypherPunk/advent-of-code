use std::fs;
use std::str::FromStr;

use ::day25::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut turing_machine = TuringMachine::from_str(&input).unwrap();

    println!(
        "What is the diagnostic checksum…? {}",
        turing_machine.get_diagnostic_checksum(),
    );
}
