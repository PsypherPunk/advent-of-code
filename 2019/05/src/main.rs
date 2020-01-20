use std::env;
use std::fs::File;
use std::io::prelude::*;

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

fn parse_opcodes(initial_state: &str) -> Vec<isize> {
    initial_state
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn get_final_state(mut opcodes: Vec<isize>, input: usize) -> Vec<isize> {
    let mut position = 0;

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
                opcodes[output_position] = input as isize;
                position += 2;
            }
            4 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => opcodes[opcodes[position + 1] as usize],
                    1 => opcodes[position + 1],
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                } as usize;
                println!("{}", p1);
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

    opcodes
}

fn main() {
    let input = read_input();
    let opcodes = parse_opcodes(&input);
    get_final_state(
        opcodes,
        env::args().nth(1).unwrap().parse::<usize>().unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100243433() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("1002,4,3,4,33")), 1),
            vec![1002, 4, 3, 4, 99],
        );
    }

    #[test]
    fn test_1101100140() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("1101,100,-1,4,0")), 1),
            vec![1101, 100, -1, 4, 99],
        );
    }

    #[test]
    fn test_jump_test_one() {
        let final_state = get_final_state(
            parse_opcodes(&String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")),
            1,
        );
        assert_eq!(final_state[13], 1);

        let final_state = get_final_state(
            parse_opcodes(&String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")),
            0,
        );
        assert_eq!(final_state[13], 0);
    }

    #[test]
    fn test_jump_test_two() {
        let final_state = get_final_state(
            parse_opcodes(&String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")),
            1,
        );
        assert_eq!(final_state[12], 1);

        let final_state = get_final_state(
            parse_opcodes(&String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")),
            0,
        );
        assert_eq!(final_state[12], 0);
    }

    #[test]
    fn test_larger_example() {
        let final_state = get_final_state(
            parse_opcodes(&String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")),
            0,
        );
        assert_eq!(final_state[32], 999);

        let final_state = get_final_state(
            parse_opcodes(&String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99")),
            8,
        );
        assert_eq!(final_state[20], 1000);
    }
}
