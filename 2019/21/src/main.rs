use std::collections::VecDeque;
use std::fs;

#[derive(Clone, Debug)]
struct Intcode {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
    inputs: VecDeque<isize>,
}

struct Springdroid {
    intcode: Intcode,
}

impl Springdroid {
    fn new(input: &str) -> Self {
        Springdroid {
            intcode: Intcode::new(input),
        }
    }

    fn run_springscript(&mut self, springscript: &str) -> Option<isize> {
        for ch in springscript.chars() {
            self.intcode.inputs.push_back(ch as isize);
        }

        while let Some(output) = self.intcode.run() {
            if output > 0 && output <= 127 {
                print!("{}", char::from(output as u8));
            } else {
                return Some(output);
            }
        }
        None
    }
}

impl Intcode {
    fn new(input: &str) -> Intcode {
        let mut opcode = parse_opcodes(input);
        let mut extension = vec![0; 10000];
        opcode.append(&mut extension);
        Intcode {
            opcode,
            position: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
        }
    }

    fn get_param1(&mut self) -> isize {
        match (self.opcode[self.position] / 100) % 10 {
            0 => self.opcode[self.opcode[self.position + 1] as usize],
            1 => self.opcode[self.position + 1],
            2 => self.opcode[(self.relative_base + self.opcode[self.position + 1]) as usize],
            _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
        }
    }

    fn get_param2(&mut self) -> isize {
        match (self.opcode[self.position] / 1000) % 10 {
            0 => self.opcode[self.opcode[self.position + 2] as usize],
            1 => self.opcode[self.position + 2],
            2 => self.opcode[(self.relative_base + self.opcode[self.position + 2]) as usize],
            _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
        }
    }

    fn get_param3(&mut self) -> usize {
        match (self.opcode[self.position] / 10000) % 10 {
            0 => self.opcode[self.position + 3] as usize,
            2 => (self.relative_base + self.opcode[self.position + 3]) as usize,
            _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
        }
    }

    fn run(&mut self) -> Option<isize> {
        loop {
            let opcode = self.opcode[self.position] % 100;
            match opcode {
                1 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output = p1 + p2;
                    let output_position = self.get_param3();
                    self.opcode[output_position] = output;
                    self.position += 4;
                }
                2 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output = p1 * p2;
                    let output_position = self.get_param3();
                    self.opcode[output_position] = output;
                    self.position += 4;
                }
                3 => {
                    let output_position = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.position + 1] as usize,
                        2 => (self.relative_base + self.opcode[self.position + 1]) as usize,
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let i = match self.inputs.pop_front() {
                        Some(i) => i as usize,
                        None => panic!("Attempt to read empty inputs!"),
                    };
                    self.opcode[output_position as usize] = i as isize;
                    self.position += 2;
                }
                4 => {
                    let p1 = self.get_param1();
                    self.position += 2;
                    return Some(p1);
                }
                5 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    if p1 != 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                6 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    if p1 == 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                7 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output_position = self.get_param3();
                    if p1 < p2 {
                        self.opcode[output_position as usize] = 1;
                    } else {
                        self.opcode[output_position as usize] = 0;
                    }
                    self.position += 4;
                }
                8 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output_position = self.get_param3();
                    if p1 == p2 {
                        self.opcode[output_position] = 1;
                    } else {
                        self.opcode[output_position] = 0;
                    }
                    self.position += 4;
                }
                9 => {
                    let p1 = self.get_param1();
                    self.relative_base += p1;
                    self.position += 2;
                }
                99 => break,
                _ => panic!(
                    "Invalid self.opcode at position {} for {:?}",
                    self.position, self.opcode
                ),
            };
        }
        None
    }
}

fn parse_opcodes(initial_state: &str) -> Vec<isize> {
    initial_state
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt.");

    let mut droid = Springdroid::new(&input.trim());
    let part_one = r#"OR A T
AND C T
NOT T J
AND D J
WALK
"#;
    if let Some(output) = droid.run_springscript(part_one) {
        println!("What amount of hull damage does it report? {}", output);
    }

    let mut droid = Springdroid::new(&input.trim());
    let part_two = r#"OR B J
AND C J
NOT J J
AND D J
AND H J
NOT A T
OR T J
RUN
"#;
    if let Some(output) = droid.run_springscript(part_two) {
        println!(
            "What amount of hull damage does the springdroid now report? {}",
            output,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
