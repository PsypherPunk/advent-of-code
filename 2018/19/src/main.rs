use std::fs;

use ::day19::*;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut cpu = Cpu::from_str(&input).unwrap();
    cpu.execute();
    println!("What value is left in register 0…? {}", cpu.registers[0]);

    println!(
        "What value is left in register 0…? {}",
        cpu.execute_part_two()
    );
}
