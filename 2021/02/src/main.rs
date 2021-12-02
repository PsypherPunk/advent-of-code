use std::fs;

use ::day02::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you multiply your final horizontal position by your final depth? {}",
        get_part_one(&input),
    );

    println!(
        "What do you get if you multiply your final horizontal position by your final depth? {}",
        get_part_two(&input),
    );
}
