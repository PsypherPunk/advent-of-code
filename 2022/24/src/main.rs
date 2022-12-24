use std::fs;

use ::day24::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the fewest number of minutes required to avoid the blizzards and reach the goal? {}",
        get_part_one(&input),
    );

    println!(
        "What is the fewest number of minutes required to reach the goal, go back to the start, then reach the goal again? {}",
        get_part_two(&input),
    );
}
