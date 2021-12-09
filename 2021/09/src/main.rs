use std::fs;

use ::day09::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the risk levels of all low points on your heightmap? {}",
        get_part_one(&input),
    );

    println!(
        "What do you get if you multiply together the sizes of the three largest basins? {}",
        get_part_two(&input),
    );
}
