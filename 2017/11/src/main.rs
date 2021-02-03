use std::fs;

use ::day11::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let (position, max) = read_path(&input);

    println!(
        "…you need to determine the fewest number of steps required to reach him… {}",
        get_steps(position),
    );

    println!("How many steps away is the furthest…? {}", max);
}
