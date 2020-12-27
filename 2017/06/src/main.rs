use std::fs;

use ::day06::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many…cycles must be completed…? {}",
        get_steps_to_repeat(&input),
    );
}
