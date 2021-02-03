use std::fs;

use ::day12::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many programs are in the group that contains program ID 0? {}",
        get_group_count_for("0", &input),
    );
}
