use std::fs;

use ::day23::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut program = Program::new(&input);
    program.run();

    println!(
        "…how many times is the mul instruction invoked? {}",
        program.mul_count,
    );
}
