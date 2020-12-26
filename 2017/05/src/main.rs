use std::fs;

use ::day05::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let offsets = get_offsets(&input);
    println!(
        "How many steps does it take to reach the exit? {}",
        get_steps_to_exit(offsets),
    );

    let offsets = get_offsets(&input);
    println!(
        "How many steps does it now take to reach the exit? {}",
        get_stranger_steps_to_exit(offsets),
    );
}
