use std::fs;

use ::day11::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…you need to determine the fewest number of steps required to reach him… {}",
        get_steps(read_path(&input)),
    );
}
