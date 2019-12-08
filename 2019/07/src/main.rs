use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

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

fn get_output(mut opcodes: Vec<isize>, mut inputs: Vec<usize>) -> usize {
    let mut position = 0;
    let mut output: usize = 0;

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
                let input = inputs.pop().unwrap();
                opcodes[output_position] = input as isize;
                position += 2;
            }
            4 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                } as usize;
                output = p1;
                position += 2;
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

    output
}

fn run_amplifier_sequence(opcodes: Vec<isize>) -> usize {
    let mut output = 0;
    let mut outputs = vec![];
    for combination in (0..=4).permutations(5) {
        for phase_setting in combination.iter() {
            output = get_output(opcodes.clone(), vec![output, *phase_setting]);
        }
        outputs.push(output);
        output = 0;
    }
    *outputs.iter().max().unwrap() as usize
}

fn main() {
    let input = read_input();
    let opcodes = parse_opcodes(&input);
    let highest_signal = run_amplifier_sequence(opcodes);
    println!(
        "What is the highest signal that can be sent to the thrusters? {}",
        highest_signal
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43210() {
        let intcode = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let opcodes = parse_opcodes(&intcode);
        assert_eq!(run_amplifier_sequence(opcodes), 43210,);
    }

    #[test]
    fn test_54321() {
        let intcode = String::from(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let opcodes = parse_opcodes(&intcode);
        assert_eq!(run_amplifier_sequence(opcodes), 54321,);
    }

    #[test]
    fn test_65210() {
        let intcode = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let opcodes = parse_opcodes(&intcode);
        assert_eq!(run_amplifier_sequence(opcodes), 65210,);
    }
}
