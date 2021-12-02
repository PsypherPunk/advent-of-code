use std::fs;

use ::day02::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What do you get if you multiply your final horizontal position by your final depth? {}",
        get_final_horizontal_depth_product(&input),
    );
}
