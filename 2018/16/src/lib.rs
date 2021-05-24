use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct InputFile {
    samples: Vec<Sample>,
    test_program: Vec<Instruction>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Sample {
    before: Vec<usize>,
    opcodes: Instruction,
    after: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    opcode: usize,
    input_a: usize,
    input_b: usize,
    output: usize,
}

enum OpCode {
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

impl Sample {
    fn is_opcode(&self, opcode: &OpCode) -> bool {
        self.get_result(opcode) == self.after[self.opcodes.output]
    }

    fn get_result(&self, opcode: &OpCode) -> usize {
        match opcode {
            OpCode::Addr => self.before[self.opcodes.input_a] + self.before[self.opcodes.input_b],
            OpCode::Addi => self.before[self.opcodes.input_a] + self.opcodes.input_b,
            OpCode::Mulr => self.before[self.opcodes.input_a] * self.before[self.opcodes.input_b],
            OpCode::Muli => self.before[self.opcodes.input_a] * self.opcodes.input_b,
            OpCode::Banr => self.before[self.opcodes.input_a] & self.before[self.opcodes.input_b],
            OpCode::Bani => self.before[self.opcodes.input_a] & self.opcodes.input_b,
            OpCode::Borr => self.before[self.opcodes.input_a] | self.before[self.opcodes.input_b],
            OpCode::Bori => self.before[self.opcodes.input_a] | self.opcodes.input_b,
            OpCode::Setr => self.before[self.opcodes.input_a],
            OpCode::Seti => self.opcodes.input_a,
            OpCode::Gtir => {
                if self.opcodes.input_a > self.before[self.opcodes.input_b] {
                    1
                } else {
                    0
                }
            }
            OpCode::Gtri => {
                if self.before[self.opcodes.input_a] > self.opcodes.input_b {
                    1
                } else {
                    0
                }
            }
            OpCode::Gtrr => {
                if self.before[self.opcodes.input_a] > self.before[self.opcodes.input_b] {
                    1
                } else {
                    0
                }
            }
            OpCode::Eqir => {
                if self.opcodes.input_a == self.before[self.opcodes.input_b] {
                    1
                } else {
                    0
                }
            }
            OpCode::Eqri => {
                if self.before[self.opcodes.input_a] == self.opcodes.input_b {
                    1
                } else {
                    0
                }
            }
            OpCode::Eqrr => {
                if self.before[self.opcodes.input_a] == self.before[self.opcodes.input_b] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl InputFile {
    pub fn get_multiple_match_count(&self) -> usize {
        let opcodes = [
            OpCode::Addr,
            OpCode::Addi,
            OpCode::Mulr,
            OpCode::Muli,
            OpCode::Banr,
            OpCode::Bani,
            OpCode::Borr,
            OpCode::Bori,
            OpCode::Setr,
            OpCode::Seti,
            OpCode::Gtir,
            OpCode::Gtri,
            OpCode::Gtrr,
            OpCode::Eqir,
            OpCode::Eqri,
            OpCode::Eqrr,
        ];

        self.samples
            .iter()
            .map(|sample| {
                opcodes
                    .iter()
                    .filter(|opcode| sample.is_opcode(opcode))
                    .count()
            })
            .filter(|opcode_count| *opcode_count >= 3)
            .count()
    }
}

peg::parser! {
    grammar wrist_device() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule csv() -> Vec<usize>
            = "[" ints:integer() ++ ", " "]" { ints }

        rule before() -> Vec<usize>
            = "Before:" _ registers:csv() { registers }

        rule after() -> Vec<usize>
            = "After:" _ registers:csv() { registers }

        rule opcodes() -> Instruction
            = ints:integer() ++ " "
                {
                    Instruction {
                        opcode: ints[0],
                        input_a: ints[1],
                        input_b: ints[2],
                        output: ints[3],
                    }
                }

        pub rule sample() -> Sample
            = b:before()
              _
              o:opcodes()
              _
              a:after()
                { Sample {before: b, opcodes: o, after: a} }

        rule samples() -> Vec<Sample>
            = s:sample() ++ _ { s }

        rule test_program() -> Vec<Instruction>
            = o:opcodes() ++ _ { o }

        pub rule input() -> InputFile
            = s:samples()
              _
              p:test_program()
              _
                { InputFile { samples: s, test_program: p } }
    }
}

impl FromStr for InputFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match wrist_device::input(&s.trim()) {
            Ok(input) => Ok(input),
            Err(_) => Err(r#"¯\_(ツ)_/¯"#.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = r#"Before: [1, 1, 3, 3]
11 1 0 1
After:  [1, 1, 3, 3]

Before: [0, 1, 2, 2]
3 2 2 1
After:  [0, 2, 2, 2]



7 2 0 0
11 0 2 0"#;

        assert!(InputFile::from_str(&input).is_ok());
    }

    #[test]
    fn test_opcodes() {
        let input = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"#;

        let sample: Sample = wrist_device::sample(&input).unwrap();

        assert!(sample.is_opcode(&OpCode::Mulr));
        assert!(sample.is_opcode(&OpCode::Addi));
        assert!(sample.is_opcode(&OpCode::Seti));
        assert_eq!(sample.is_opcode(&OpCode::Addr), false);
    }
}
