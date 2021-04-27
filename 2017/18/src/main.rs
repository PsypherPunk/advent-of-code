use std::fs;

use ::day18::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut zero = Program::new(&input, 0);
    let mut one = Program::new(&input, 1);

    Program::duet(&mut zero, &mut one);

    println!(
        "What is the value of the recovered frequency…? {}",
        zero.first_non_zero_recv,
    );

    println!(
        "…how many times did program 1 send a value? {}",
        one.sent_count,
    );
}
