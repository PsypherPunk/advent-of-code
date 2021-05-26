use std::str::FromStr;

pub struct Cpu {
    pub registers: [usize; 6],
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
    input_a: usize,
    input_b: usize,
    output: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl FromStr for Cpu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match wrist_device::cpu(&s.trim()) {
            Ok(input) => Ok(input),
            Err(_) => Err(r#"¯\_(ツ)_/¯"#.to_string()),
        }
    }
}

impl Cpu {
    fn get_instruction_pointer(&self) -> usize {
        self.registers[self.instruction_pointer]
    }

    pub fn execute(&mut self) {
        while self.get_instruction_pointer() < self.instructions.len() {
            let instruction = self.instructions[self.get_instruction_pointer()];
            let output = self.get_output(&instruction);
            self.registers[instruction.output] = output;
            self.registers[self.instruction_pointer] += 1;
        }
    }

    fn get_output(&self, instruction: &Instruction) -> usize {
        match instruction.opcode {
            OpCode::Addr => {
                self.registers[instruction.input_a] + self.registers[instruction.input_b]
            }
            OpCode::Addi => self.registers[instruction.input_a] + instruction.input_b,
            OpCode::Mulr => {
                self.registers[instruction.input_a] * self.registers[instruction.input_b]
            }
            OpCode::Muli => self.registers[instruction.input_a] * instruction.input_b,
            OpCode::Banr => {
                self.registers[instruction.input_a] & self.registers[instruction.input_b]
            }
            OpCode::Bani => self.registers[instruction.input_a] & instruction.input_b,
            OpCode::Borr => {
                self.registers[instruction.input_a] | self.registers[instruction.input_b]
            }
            OpCode::Bori => self.registers[instruction.input_a] | instruction.input_b,
            OpCode::Setr => self.registers[instruction.input_a],
            OpCode::Seti => instruction.input_a,
            OpCode::Gtir => {
                if instruction.input_a > self.registers[instruction.input_b] {
                    1
                } else {
                    0
                }
            }
            OpCode::Gtri => {
                if self.registers[instruction.input_a] > instruction.input_b {
                    1
                } else {
                    0
                }
            }
            OpCode::Gtrr => {
                if self.registers[instruction.input_a] > self.registers[instruction.input_b] {
                    1
                } else {
                    0
                }
            }
            OpCode::Eqir => {
                if instruction.input_a == self.registers[instruction.input_b] {
                    1
                } else {
                    0
                }
            }
            OpCode::Eqri => {
                if self.registers[instruction.input_a] == instruction.input_b {
                    1
                } else {
                    0
                }
            }
            OpCode::Eqrr => {
                if self.registers[instruction.input_a] == self.registers[instruction.input_b] {
                    1
                } else {
                    0
                }
            }
        }
    }

    /// let mut a = 0;
    /// let c = 10_551_418;
    /// for b in 1..=c {
    ///     for e in 1..=c {
    ///         if b * e == c {
    ///             a += b;
    ///         }
    ///     }
    /// }
    pub fn execute_part_two(&self) -> usize {
        let c = 10_551_418;

        (1..=c).filter(|divisor| c / *divisor * *divisor == c).sum()
    }
}

peg::parser! {
    grammar wrist_device() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule opcode() -> OpCode
            = o:$("addr" / "addi" / "mulr" / "muli" / "banr" / "bani" / "borr" / "bori" / "setr" / "seti" / "gtir" / "gtri" / "gtrr" / "eqir" / "eqri" / "eqrr")
                {
                    match o {
                        "addr" => OpCode::Addr,
                        "addi" => OpCode::Addi,
                        "mulr" => OpCode::Mulr,
                        "muli" => OpCode::Muli,
                        "banr" => OpCode::Banr,
                        "bani" => OpCode::Bani,
                        "borr" => OpCode::Borr,
                        "bori" => OpCode::Bori,
                        "setr" => OpCode::Setr,
                        "seti" => OpCode::Seti,
                        "gtir" => OpCode::Gtir,
                        "gtri" => OpCode::Gtri,
                        "gtrr" => OpCode::Gtrr,
                        "eqir" => OpCode::Eqir,
                        "eqri" => OpCode::Eqri,
                        "eqrr" => OpCode::Eqrr,
                        _ => panic!(r#"¯\_(ツ)_/¯"#),
                    }
                }

        rule instructions() -> Instruction
            = opcode:opcode()
              _
              ints:integer() ++ _
                {
                    Instruction {
                        opcode,
                        input_a: ints[0],
                        input_b: ints[1],
                        output: ints[2],
                    }
                }

        rule ip() -> usize
            = "#ip" _ i:integer()
                { i }

        pub rule cpu() -> Cpu
            = ip:ip()
              _
              instructions:instructions() ++ _
                {
                    Cpu {
                        registers: [0; 6],
                        instructions,
                        instruction_pointer: ip,
                    }
                }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#;

        let mut cpu = Cpu::from_str(&input).unwrap();
        cpu.execute();

        assert_eq!([5, 6, 0, 0, 9], cpu.registers[1..]);
    }
}
