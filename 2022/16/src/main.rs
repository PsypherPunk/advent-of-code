use std::fs;

use ::day16::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the most pressure you can release? {}",
        get_part_one(&input),
    );

    println!(
        "With you and an elephant working together for 26 minutes, what is the most pressure you could release? {}",
        get_part_two(&input),
    );
}
