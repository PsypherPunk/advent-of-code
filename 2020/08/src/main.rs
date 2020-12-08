use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Operation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
    Seen,
}

#[derive(Clone, Debug)]
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
    fn run(&mut self) -> Result<isize, ()> {
        let mut position: isize = 0;
        while position < self.boot_code.len() as isize {
            let operation = self.boot_code.get(position as usize).unwrap();
            let offset = match operation {
                Operation::Nop(_) => 1,
                Operation::Acc(argument) => {
                    self.accumulator += argument;
                    1
                }
                Operation::Jmp(argument) => *argument,
                Operation::Seen => return Err(()),
            };
            self.boot_code[position as usize] = Operation::Seen;
            position += offset;
        }
        Ok(self.accumulator)
    }

    fn get_nops(&self) -> Vec<usize> {
        self.boot_code
            .iter()
            .enumerate()
            .filter(|(_, operation)| matches!(operation, Operation::Nop(_)))
            .map(|(index, _)| index)
            .collect::<Vec<_>>()
    }

    fn get_jmps(&self) -> Vec<usize> {
        self.boot_code
            .iter()
            .enumerate()
            .filter(|(_, operation)| matches!(operation, Operation::Jmp(_)))
            .map(|(index, _)| index)
            .collect::<Vec<_>>()
    }

    fn run_with_changes(&mut self) -> isize {
        let nops = self.get_nops();
        let jmps = self.get_jmps();

        let backup = self.boot_code.clone();

        for nop in nops {
            self.boot_code = backup.clone();
            self.accumulator = 0;
            self.boot_code[nop] = match self.boot_code[nop] {
                Operation::Nop(argument) => Operation::Jmp(argument),
                _ => panic!("Invalid operation found at index {}.", nop),
            };
            if let Ok(accumulator) = self.run() {
                return accumulator;
            }
        }

        for jmp in jmps {
            self.boot_code = backup.clone();
            self.accumulator = 0;
            self.boot_code[jmp] = match self.boot_code[jmp] {
                Operation::Jmp(argument) => Operation::Nop(argument),
                _ => panic!("Invalid operation found at index {}.", jmp),
            };
            if let Ok(accumulator) = self.run() {
                return accumulator;
            }
        }

        panic!("Unable to find nop/jmp replacement.");
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut boot_code = BootCode::from_str(&input).unwrap();
    boot_code.run().unwrap_err();

    println!(
        "…what value is in the accumulator? {}",
        boot_code.accumulator,
    );

    let mut boot_code = BootCode::from_str(&input).unwrap();
    boot_code.run_with_changes();

    println!(
        "What is the value of the accumulator after the program terminates? {}",
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
        boot_code.run().unwrap_err();

        assert_eq!(5, boot_code.accumulator);
    }

    #[test]
    fn test_part_two() {
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
        boot_code.run_with_changes();

        assert_eq!(8, boot_code.accumulator);
    }
}
