use std::fs;

use ::day13::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the severity of your whole trip? {}",
        get_severity(&input),
    );
}
