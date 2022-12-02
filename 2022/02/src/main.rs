use std::fs;

use ::day02::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What would your total score be if everything goes exactly according to your strategy guide? {}",
        get_part_one(&input),
    );

    println!(
        "…what would your total score be if everything goes exactly according to your strategy guide? {}",
        get_part_two(&input),
    );
}
