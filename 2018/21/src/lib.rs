use std::collections::HashSet;
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

    /// Execute instructions to completion.
    ///
    /// For *Part One*, the instructions make an `eqrr`, comparing the
    /// values of register `0` to that of register `5`, to determine
    /// whether or not to continue: the value in the latter is
    /// therefore our answer.
    pub fn get_register_zero_halt(&mut self) -> Result<usize, String> {
        while self.get_instruction_pointer() < self.instructions.len() {
            let instruction = self.instructions[self.get_instruction_pointer()];

            if matches!(instruction.opcode, OpCode::Eqrr) {
                return Ok(self.registers[instruction.input_a]);
            }

            let output = self.get_output(&instruction);
            self.registers[instruction.output] = output;
            self.registers[self.instruction_pointer] += 1;
        }

        Err("".to_owned())
    }

    /// Execute instructions to completion.
    ///
    /// For *Part Two*, the pattern for *Part One* holds true. However,
    /// this time we're interested not in that initial value for
    /// register `5` but the value before it finally loops.
    pub fn execute_part_two(&mut self) -> Result<usize, String> {
        let mut seen = HashSet::new();
        let mut previous = 0;

        while self.get_instruction_pointer() < self.instructions.len() {
            let instruction = self.instructions[self.get_instruction_pointer()];

            if matches!(instruction.opcode, OpCode::Eqrr) {
                if !seen.insert(self.registers[instruction.input_a]) {
                    return Ok(previous);
                }
                previous = self.registers[instruction.input_a];
            }

            let output = self.get_output(&instruction);
            self.registers[instruction.output] = output;
            self.registers[self.instruction_pointer] += 1;
        }

        Err("".to_owned())
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
