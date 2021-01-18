use std::fs;

use ::day08::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let cpu = get_cpu(&input);

    println!(
        "What is the largest value in any register…? {}",
        &cpu.registers.values().max().unwrap(),
    );

    println!(
        "…the highest value held in any register during this process… {}",
        &cpu.highest,
    );
}
