use std::fs;

use ::day06::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many characters need to be processed before the first start-of-packet marker is detected? {}",
        get_part_one(&input),
    );

    println!(
        "{}",
        get_part_two(&input),
    );
}
