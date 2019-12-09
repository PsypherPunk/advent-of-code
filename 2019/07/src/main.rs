use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

#[derive(Clone)]
struct Amplifier {
    opcode: Vec<isize>,
    position: usize,
}

impl Amplifier {
    fn new(input: &String) -> Amplifier {
        let opcode = parse_opcodes(input);
        Amplifier {
            opcode,
            position: 0,
        }
    }

    fn run(&mut self, mut phase_setting: Option<&i32>, signal: usize) -> Option<usize> {
        while self.position < self.opcode.len() {
            let opcode = self.opcode[self.position] % 100;
            match opcode {
                1 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let p2 = match (self.opcode[self.position] / 1000) % 10 {
                        0 => self.opcode[self.opcode[self.position + 2] as usize],
                        1 => self.opcode[self.position + 2],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let output = p1 + p2;
                    let output_position = self.opcode[self.position + 3] as usize;
                    self.opcode[output_position] = output;
                    self.position += 4;
                }
                2 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let p2 = match (self.opcode[self.position] / 1000) % 10 {
                        0 => self.opcode[self.opcode[self.position + 2] as usize],
                        1 => self.opcode[self.position + 2],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let output = p1 * p2;
                    let output_position = self.opcode[self.position + 3] as usize;
                    self.opcode[output_position] = output;
                    self.position += 4;
                }
                3 => {
                    let output_position = self.opcode[self.position + 1] as usize;
                    let i = match phase_setting.take() {
                        Some(i) => *i as usize,
                        None => signal,
                    };
                    self.opcode[output_position] = i as isize;
                    self.position += 2;
                }
                4 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    } as usize;
                    self.position += 2;
                    return Some(p1);
                }
                5 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let p2 = match (self.opcode[self.position] / 1000) % 10 {
                        0 => self.opcode[self.opcode[self.position + 2] as usize],
                        1 => self.opcode[self.position + 2],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    if p1 != 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                6 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let p2 = match (self.opcode[self.position] / 1000) % 10 {
                        0 => self.opcode[self.opcode[self.position + 2] as usize],
                        1 => self.opcode[self.position + 2],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    if p1 == 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                7 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let p2 = match (self.opcode[self.position] / 1000) % 10 {
                        0 => self.opcode[self.opcode[self.position + 2] as usize],
                        1 => self.opcode[self.position + 2],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let output_position = self.opcode[self.position + 3] as usize;
                    if p1 < p2 {
                        self.opcode[output_position] = 1;
                    } else {
                        self.opcode[output_position] = 0;
                    }
                    self.position += 4;
                }
                8 => {
                    let p1 = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.opcode[self.position + 1] as usize],
                        1 => self.opcode[self.position + 1],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let p2 = match (self.opcode[self.position] / 1000) % 10 {
                        0 => self.opcode[self.opcode[self.position + 2] as usize],
                        1 => self.opcode[self.position + 2],
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let output_position = self.opcode[self.position + 3] as usize;
                    if p1 == p2 {
                        self.opcode[output_position] = 1;
                    } else {
                        self.opcode[output_position] = 0;
                    }
                    self.position += 4;
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

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn parse_opcodes(initial_state: &String) -> Vec<isize> {
    initial_state
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn get_output(opcodes: &mut Vec<isize>, mut inputs: Vec<usize>, mut position: usize) -> usize {
    let mut input;

    while position < opcodes.len() {
        let opcode = opcodes[position] % 100;
        match opcode {
            1 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => opcodes[opcodes[position + 2] as usize],
                    1 => opcodes[position + 2],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let output = p1 + p2;
                let output_position = opcodes[position + 3] as usize;
                opcodes[output_position] = output;
                position += 4;
            }
            2 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => opcodes[opcodes[position + 2] as usize],
                    1 => opcodes[position + 2],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let output = p1 * p2;
                let output_position = opcodes[position + 3] as usize;
                opcodes[output_position] = output;
                position += 4;
            }
            3 => {
                let output_position = opcodes[position + 1] as usize;
                input = match inputs.pop() {
                    Some(i) => i,
                    None => panic!("No inputs!"),
                };
                opcodes[output_position] = input as isize;
                position += 2;
            }
            4 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                } as usize;
                position += 2;
                return p1;
            }
            5 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => opcodes[opcodes[position + 2] as usize],
                    1 => opcodes[position + 2],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                if p1 != 0 {
                    position = p2 as usize;
                } else {
                    position += 3;
                }
            }
            6 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => opcodes[opcodes[position + 2] as usize],
                    1 => opcodes[position + 2],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                if p1 == 0 {
                    position = p2 as usize;
                } else {
                    position += 3;
                }
            }
            7 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => opcodes[opcodes[position + 2] as usize],
                    1 => opcodes[position + 2],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let output_position = opcodes[position + 3] as usize;
                if p1 < p2 {
                    opcodes[output_position] = 1;
                } else {
                    opcodes[output_position] = 0;
                }
                position += 4;
            }
            8 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => opcodes[opcodes[position + 2] as usize],
                    1 => opcodes[position + 2],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let output_position = opcodes[position + 3] as usize;
                if p1 == p2 {
                    opcodes[output_position] = 1;
                } else {
                    opcodes[output_position] = 0;
                }
                position += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode at position {} for {:?}", position, opcodes),
        };
    }
    0
}

fn run_amplifier_sequence(opcodes: &Vec<isize>) -> usize {
    let mut output = 0;
    let mut outputs = vec![];
    for combination in (0..=4).permutations(5) {
        for phase_setting in combination.iter() {
            output = get_output(opcodes.clone().as_mut(), vec![output, *phase_setting], 0);
        }
        outputs.push(output);
        output = 0;
    }
    *outputs.iter().max().unwrap() as usize
}

fn run_feedback_loop(input: &String) -> usize {
    let mut outputs: Vec<usize> = Vec::new();

    for combination in (5..=9).permutations(5) {
        let mut signal: usize = 0;
        let mut phase_settings = combination.iter().clone();

        let mut amplifiers = vec![Amplifier::new(input); 5];

        for i in (0..amplifiers.len()).cycle() {
            signal = match amplifiers[i].run(phase_settings.next(), signal) {
                Some(s) => s,
                None => break,
            }
        }
        outputs.push(signal);
    }
    *outputs.iter().max().unwrap()
}

fn main() {
    let input = read_input();
    let opcodes = parse_opcodes(&input);
    let highest_signal = run_amplifier_sequence(&opcodes);
    println!(
        "What is the highest signal that can be sent to the thrusters? {}",
        highest_signal
    );
    let feedback_signal = run_feedback_loop(&input);
    println!(
        "What is the highest signal that can be sent to the thrusters? {}",
        feedback_signal,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43210() {
        let intcode = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let opcodes = parse_opcodes(&intcode);
        assert_eq!(run_amplifier_sequence(&opcodes), 43210);
    }

    #[test]
    fn test_54321() {
        let intcode = String::from(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let opcodes = parse_opcodes(&intcode);
        assert_eq!(run_amplifier_sequence(&opcodes), 54321);
    }

    #[test]
    fn test_65210() {
        let intcode = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let opcodes = parse_opcodes(&intcode);
        assert_eq!(run_amplifier_sequence(&opcodes), 65210);
    }

    #[test]
    fn test_139629729() {
        let intcode = String::from(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(run_feedback_loop(&intcode), 139629729);
    }

    #[test]
    fn test_18216() {
        let intcode = String::from(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(run_feedback_loop(&intcode), 18216);
    }
}
