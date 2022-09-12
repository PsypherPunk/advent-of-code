use std::fs;

use ::day06::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the size of the largest area…? {}",
        get_part_one(&input),
    );

    println!(
        "What is the size of the region containing all locations which have a total distance to all given coordinates of less than 10000? {}",
        get_part_two(&input, 10_000),
    );
}
