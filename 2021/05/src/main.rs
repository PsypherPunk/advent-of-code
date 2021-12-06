use std::fs;

use ::day05::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "At how many points do at least two lines overlap? {}",
        get_part_one(&input),
    );

    println!(
        "At how many points do at least two lines overlap? {}",
        get_part_two(&input),
    );
}
