use std::fs;

use ::day24::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let components = get_components(&input);
    let mut bridges = get_bridges(&components);

    println!(
        "What is the strength of the strongest bridge you can make…? {}",
        get_strongest_bridge(&mut bridges),
    );

    println!(
        "What is the strength of the longest bridge you can make? {}",
        get_strongest_longest_bridge(&mut bridges),
    );
}
