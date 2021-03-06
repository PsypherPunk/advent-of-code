use std::fs;

use ::day21::*;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut cpu = Cpu::from_str(&input).unwrap();
    println!(
        "What is the lowest…integer…that causes the program to halt…the fewest instructions? {}",
        cpu.get_register_zero_halt().unwrap(),
    );

    let mut cpu = Cpu::from_str(&input).unwrap();
    println!(
        "What is the lowest…integer…that causes the program to halt after…the most instructions? {}",
        cpu.execute_part_two().unwrap(),
    );
}
