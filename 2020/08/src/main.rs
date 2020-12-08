use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
    Seen,
}

#[derive(Debug)]
struct BootCode {
    accumulator: isize,
    boot_code: Vec<Operation>,
}

impl FromStr for BootCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(
                |line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                    [instruction, argument] => {
                        let offset = argument.parse().unwrap();
                        match instruction {
                            "nop" => Operation::Nop(offset),
                            "acc" => Operation::Acc(offset),
                            "jmp" => Operation::Jmp(offset),
                            _ => panic!("Invalid instruction: {}", instruction),
                        }
                    }
                    _ => panic!("Invalid line: {}", line),
                },
            )
            .collect::<Vec<_>>();

        Ok(Self {
            accumulator: 0,
            boot_code: instructions,
        })
    }
}

impl BootCode {
    fn run_until_repeat(&mut self) {
        let mut position: isize = 0;
        loop {
            let operation = self.boot_code.get(position as usize).unwrap();
            let offset = match operation {
                Operation::Nop(_) => 1,
                Operation::Acc(argument) => {
                    self.accumulator += argument;
                    1
                }
                Operation::Jmp(argument) => *argument,
                Operation::Seen => {
                    break;
                }
            };
            self.boot_code[position as usize] = Operation::Seen;
            position += offset;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut boot_code = BootCode::from_str(&input).unwrap();
    boot_code.run_until_repeat();

    println!(
        "…what value is in the accumulator? {}",
        boot_code.accumulator,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

        let mut boot_code = BootCode::from_str(&input).unwrap();
        boot_code.run_until_repeat();

        assert_eq!(5, boot_code.accumulator);
    }
}
