use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Clone)]
enum Instruction {
    Snd(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

pub struct Program {
    registers: HashMap<char, isize>,
    instructions: Vec<Instruction>,
    position: usize,
    queue: VecDeque<isize>,
    last_sent: isize,
    pub sent_count: usize,
    pub first_non_zero_recv: isize,
}

impl Program {
    pub fn new(input: &str, id: isize) -> Program {
        let instructions = input
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<Instruction>>();

        let mut program = Program {
            registers: HashMap::new(),
            instructions,
            position: 0,
            queue: VecDeque::new(),
            sent_count: 0,
            last_sent: 0,
            first_non_zero_recv: 0,
        };
        program.registers.entry('p').or_insert(id);
        program
    }

    pub fn duet(zero: &mut Program, one: &mut Program) {
        zero.run(one, true);
    }

    pub fn run(&mut self, other: &mut Program, first: bool) {
        while self.position < self.instructions.len() {
            let advance = self.instructions[self.position]
                .clone()
                .execute(self, other);
            if !advance {
                if !first {
                    break;
                }
                other.run(self, false);
                if !self.instructions[self.position]
                    .clone()
                    .execute(self, other)
                {
                    break;
                }
            }
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
            Value::Register(c) => program.registers[&c],
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
    fn execute(&self, program: &mut Program, other: &mut Program) -> bool {
        program.position += 1;
        match *self {
            Instruction::Snd(ref v) => {
                let v = v.get_value(program);
                program.last_sent = v;
                other.queue.push_back(v);
                program.sent_count += 1;
            }
            Instruction::Set(ref r, ref v) => {
                program.update_register(&r.get_register(), v.get_value(program));
            }
            Instruction::Add(ref r, ref v) => {
                let add = program.get_register(&r.get_register()) + v.get_value(program);
                program.update_register(&r.get_register(), add);
            }
            Instruction::Mul(ref r, ref v) => {
                let mul = program.get_register(&r.get_register()) * v.get_value(program);
                program.update_register(&r.get_register(), mul);
            }
            Instruction::Mod(ref r, ref v) => {
                let mod_ = program.get_register(&r.get_register()) % v.get_value(program);
                program.update_register(&r.get_register(), mod_);
            }
            Instruction::Rcv(ref r) => {
                if program.first_non_zero_recv == 0 && program.get_register(&r.get_register()) != 0
                {
                    program.first_non_zero_recv = program.last_sent;
                }
                if let Some(v) = program.queue.pop_front() {
                    program.update_register(&r.get_register(), v);
                } else {
                    program.position -= 1;
                    return false;
                }
            }
            Instruction::Jgz(ref t, ref o) => {
                if t.get_value(program) > 0 {
                    program.position =
                        (program.position as isize + o.get_value(program) - 1) as usize;
                }
            }
        }
        true
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(match parts[0] {
            "snd" => Instruction::Snd(parts[1].parse().unwrap()),
            "set" => Instruction::Set(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "add" => Instruction::Add(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "mul" => Instruction::Mul(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "mod" => Instruction::Mod(Value::from_str(parts[1])?, Value::from_str(parts[2])?),
            "rcv" => Instruction::Rcv(Value::from_str(parts[1])?),
            "jgz" => Instruction::Jgz(parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            _ => {
                return Err(format!("Invalid instruction: {}", parts[0]));
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#;
        let mut zero = Program::new(&input, 0);
        let mut one = Program::new(&input, 1);

        Program::duet(&mut zero, &mut one);

        assert_eq!(4, zero.first_non_zero_recv);
    }

    #[test]
    fn test_part_two() {
        let input = r#"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"#;
        let mut zero = Program::new(&input, 0);
        let mut one = Program::new(&input, 1);

        Program::duet(&mut zero, &mut one);

        assert_eq!(3, one.sent_count);
    }
}
