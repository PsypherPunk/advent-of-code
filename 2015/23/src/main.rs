use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct RegisterOffset {
    register: String,
    offset: isize,
}

#[derive(Debug)]
enum Instruction {
    Half(String),
    Triple(String),
    Increment(String),
    Jump(isize),
    JumpIfEven(RegisterOffset),
    JumpIfOdd(RegisterOffset),
}

fn get_registers() -> HashMap<String, usize> {
    [(String::from("a"), 0), (String::from("b"), 0)]
        .iter()
        .cloned()
        .collect()
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let instruction = line.split_whitespace().collect::<Vec<&str>>();
            match instruction[0] {
                "hlf" => Instruction::Half(String::from(instruction[1])),
                "tpl" => Instruction::Triple(String::from(instruction[1])),
                "inc" => Instruction::Increment(String::from(instruction[1])),
                "jmp" => Instruction::Jump(instruction[1].parse().unwrap()),
                "jie" => Instruction::JumpIfEven(RegisterOffset {
                    register: instruction[1][..=0].to_string(),
                    offset: instruction[2].parse().unwrap(),
                }),
                "jio" => Instruction::JumpIfOdd(RegisterOffset {
                    register: instruction[1][..=0].to_string(),
                    offset: instruction[2].parse().unwrap(),
                }),
                _ => panic!("Invalid instruction: {}", instruction[0]),
            }
        })
        .collect()
}

fn run_program(registers: &mut HashMap<String, usize>, instructions: Vec<Instruction>) {
    let mut offset: isize = 0;
    while (offset as usize) < instructions.len() {
        let instruction = instructions.get(offset as usize).unwrap();

        match instruction {
            Instruction::Half(register) => {
                let value = registers.entry(register.clone()).or_insert(0);
                *value /= 2;
                offset += 1;
            }
            Instruction::Triple(register) => {
                let value = registers.entry(register.clone()).or_insert(0);
                *value *= 3;
                offset += 1;
            }
            Instruction::Increment(register) => {
                let value = registers.entry(register.clone()).or_insert(0);
                *value += 1;
                offset += 1;
            }
            Instruction::Jump(to_offset) => {
                offset += *to_offset;
            }
            Instruction::JumpIfEven(register_offset) => {
                let value = registers
                    .entry(register_offset.register.clone())
                    .or_insert(0);
                if *value % 2 == 0 {
                    offset += register_offset.offset;
                } else {
                    offset += 1;
                }
            }
            Instruction::JumpIfOdd(register_offset) => {
                let value = registers
                    .entry(register_offset.register.clone())
                    .or_insert(0);
                if *value == 1 {
                    offset += register_offset.offset;
                } else {
                    offset += 1;
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut registers = get_registers();
    let instructions = get_instructions(&input);

    run_program(&mut registers, instructions);

    println!(
        "What is the value in register b when the program in your puzzle input is finished executing? {}",
        registers.get("b").unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"inc a
jio a, +2
tpl a
inc a";
        let mut registers = get_registers();
        let instructions = get_instructions(&input);

        run_program(&mut registers, instructions);

        assert_eq!(2, *registers.get("a").unwrap());
    }
}
