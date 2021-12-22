use std::fs;

use ::day22::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("…how many cubes are on? {}", get_part_one(&input),);

    println!("{}", get_part_two(&input),);
}
