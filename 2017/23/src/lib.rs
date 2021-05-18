use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
enum Instruction {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

pub struct Program {
    registers: HashMap<char, isize>,
    instructions: Vec<Instruction>,
    position: usize,
    pub mul_count: usize,
}

impl Program {
    pub fn new(input: &str) -> Program {
        let instructions = input
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<Instruction>>();

        Self {
            registers: HashMap::new(),
            instructions,
            position: 0,
            mul_count: 0,
        }
    }

    pub fn run(&mut self) {
        while self.position < self.instructions.len() {
            self.instructions[self.position].clone().execute(self);
        }
    }

    fn get_register(&mut self, register: &char) -> isize {
        let current = self.registers.entry(*register).or_insert(0);
        *current
    }

    fn update_register(&mut self, register: &char, value: isize) {
        let current = self.registers.entry(*register).or_insert(0);
        *current = value;
    }
}

#[derive(Clone)]
enum Value {
    Register(char),
    Number(isize),
}

impl Value {
    fn get_value(&self, program: &Program) -> isize {
        match *self {
            Value::Register(c) => match program.registers.get(&c) {
                Some(value) => *value,
                None => 0,
            },
            Value::Number(n) => n,
        }
    }

    fn get_register(&self) -> char {
        match *self {
            Value::Register(c) => c,
            Value::Number(_) => panic!("Invalid value!"),
        }
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Value, String> {
        if let Ok(n) = s.parse() {
            Ok(Value::Number(n))
        } else {
            Ok(Value::Register(s.chars().next().unwrap()))
        }
    }
}

impl Instruction {
    fn execute(&self, program: &mut Program) {
        program.position += 1;
        match *self {
            Instruction::Set(ref x, ref y) => {
                program.update_register(&x.get_register(), y.get_value(program));
            }
            Instruction::Sub(ref x, ref y) => {
                let add = program.get_register(&x.get_register()) - y.get_value(program);
                program.update_register(&x.get_register(), add);
            }
            Instruction::Mul(ref x, ref y) => {
                let mul = program.get_register(&x.get_register()) * y.get_value(program);
                program.update_register(&x.get_register(), mul);
                program.mul_count += 1;
            }
            Instruction::Jnz(ref x, ref y) => {
                if x.get_value(program) != 0 {
                    program.position =
                        (program.position as isize + y.get_value(program) - 1) as usize;
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(match parts[0] {
            "set" => Instruction::Set(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "sub" => Instruction::Sub(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "mul" => Instruction::Mul(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "jnz" => Instruction::Jnz(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            _ => {
                return Err(format!("Invalid instruction: {}", parts[0]));
            }
        })
    }
}
