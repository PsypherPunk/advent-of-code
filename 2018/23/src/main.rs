use std::fs;

use ::day23::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let nanobots = wrist_device::nanobots(&input.trim()).unwrap();
    let strongest = get_strongest_nanobot(&nanobots);

    println!(
        "How many nanobots are in range…? {}",
        get_nanobots_in_range(&strongest, &nanobots),
    );
}
