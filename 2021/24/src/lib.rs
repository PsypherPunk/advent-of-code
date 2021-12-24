//! + 16
//! + 3
//! + 2
//! + 7
//! - 10
//! + 6
//! - 14
//! + 11
//! - 4
//! - 3
//! + 11
//! - 3
//! - 9
//! - 12
//!
//! input[4]  = input[3] + 7 - 10
//! input[6]  = input[5] + 6 - 14
//! input[8]  = input[7] + 11 - 4
//! input[9]  = input[2] + 2 - 3
//! input[11] = input[10] + 11 - 3
//! input[12] = input[1] + 3 - 9
//! input[13] = input[0] + 16 - 12
//!
//! 01234567890abc
//! 59996912981939
//! 17241911811915
//!
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Clone, Debug)]
enum IntegerVariable {
    Variable(char),
    Integer(isize),
}

#[derive(Clone, Debug)]
enum Instruction {
    Inp(char),
    Add(char, IntegerVariable),
    Mul(char, IntegerVariable),
    Div(char, IntegerVariable),
    Mod(char, IntegerVariable),
    Eql(char, IntegerVariable),
}

#[derive(Clone, Debug)]
struct Alu {
    processing_unit: HashMap<char, isize>,
    instructions: Vec<Instruction>,
}

impl FromStr for IntegerVariable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let integer_variable = match s.parse::<isize>() {
            Ok(i) => IntegerVariable::Integer(i),
            Err(_) => IntegerVariable::Variable(s.chars().next().unwrap()),
        };

        Ok(integer_variable)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = s.split_whitespace().collect::<Vec<_>>();
        let instruction = match instruction[0] {
            "inp" => Instruction::Inp(instruction[1].chars().next().unwrap()),
            "add" => Instruction::Add(
                instruction[1].chars().next().unwrap(),
                IntegerVariable::from_str(instruction[2]).unwrap(),
            ),
            "mul" => Instruction::Mul(
                instruction[1].chars().next().unwrap(),
                IntegerVariable::from_str(instruction[2]).unwrap(),
            ),
            "div" => Instruction::Div(
                instruction[1].chars().next().unwrap(),
                IntegerVariable::from_str(instruction[2]).unwrap(),
            ),
            "mod" => Instruction::Mod(
                instruction[1].chars().next().unwrap(),
                IntegerVariable::from_str(instruction[2]).unwrap(),
            ),
            "eql" => Instruction::Eql(
                instruction[1].chars().next().unwrap(),
                IntegerVariable::from_str(instruction[2]).unwrap(),
            ),
            _ => panic!(),
        };
        Ok(instruction)
    }
}

impl FromStr for Alu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect();

        Ok(Self {
            processing_unit: HashMap::new(),
            instructions,
        })
    }
}

impl Alu {
    fn execute(&mut self, inputs: &mut VecDeque<isize>) {
        for instruction in self.instructions.iter() {
            dbg!(&self.processing_unit);
            match instruction {
                Instruction::Inp(variable) => {
                    self.processing_unit
                        .insert(*variable, inputs.pop_front().unwrap());
                }
                Instruction::Add(variable, iv) => {
                    let &value = match iv {
                        IntegerVariable::Variable(v) => self.processing_unit.entry(*v).or_insert(0),
                        IntegerVariable::Integer(i) => i,
                    };
                    let entry = self.processing_unit.entry(*variable).or_insert(0);
                    *entry += value;
                }
                Instruction::Mul(variable, iv) => {
                    let &value = match iv {
                        IntegerVariable::Variable(v) => self.processing_unit.entry(*v).or_insert(0),
                        IntegerVariable::Integer(i) => i,
                    };
                    let entry = self.processing_unit.entry(*variable).or_insert(0);
                    *entry *= value;
                }
                Instruction::Div(variable, iv) => {
                    let &value = match iv {
                        IntegerVariable::Variable(v) => self.processing_unit.entry(*v).or_insert(0),
                        IntegerVariable::Integer(i) => i,
                    };
                    let entry = self.processing_unit.entry(*variable).or_insert(0);
                    *entry /= value;
                }
                Instruction::Mod(variable, iv) => {
                    let &value = match iv {
                        IntegerVariable::Variable(v) => self.processing_unit.entry(*v).or_insert(0),
                        IntegerVariable::Integer(i) => i,
                    };
                    let entry = self.processing_unit.entry(*variable).or_insert(0);
                    *entry %= value;
                }
                Instruction::Eql(variable, iv) => {
                    let &value = match iv {
                        IntegerVariable::Variable(v) => self.processing_unit.entry(*v).or_insert(0),
                        IntegerVariable::Integer(i) => i,
                    };
                    let entry = self.processing_unit.entry(*variable).or_insert(0);
                    *entry = (*entry == value) as isize;
                }
            }
        }
    }
}

/// Pure implementation of the ALU.
///
/// Clearly will never work but hey…
pub fn get_part_one(input: &str) -> isize {
    let mut alu = Alu::from_str(input).unwrap();

    (11111111111111_usize..=99999999999999)
        .rev()
        .map(|model| model.to_string())
        .filter(|model| !model.contains('0'))
        .find(|model| {
            let mut digits = model
                .chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect();
            alu.execute(&mut digits);

            *alu.processing_unit.get(&'z').unwrap() == 0
        })
        .unwrap()
        .parse()
        .unwrap()
}

pub fn get_part_two(input: &str) -> usize {
    let mut alu = Alu::from_str(input).unwrap();

    (11111111111111_usize..=99999999999999)
        .map(|model| model.to_string())
        .filter(|model| !model.contains('0'))
        .find(|model| {
            let mut digits = model
                .chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect();
            alu.execute(&mut digits);

            *alu.processing_unit.get(&'z').unwrap() == 0
        })
        .unwrap()
        .parse()
        .unwrap()
}
