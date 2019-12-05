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

fn parse_opcodes(initial_state: &String) -> Vec<isize> {
    initial_state
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn get_final_state(mut opcodes: Vec<isize>) -> Vec<isize> {
    let mut position = 0;

    while position < opcodes.len() {
        let opcode = opcodes[position] % 100;
        match opcode {
            1 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => {
                        opcodes[opcodes[position + 1] as usize]
                    },
                    1 => {
                        opcodes[position + 1]
                    },
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => {
                        opcodes[opcodes[position + 2] as usize]
                    },
                    1 => {
                        opcodes[position + 2]
                    },
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let output = p1 + p2;
                let output_position = opcodes[position + 3] as usize;
                opcodes[output_position] = output;
                position += 4;
            },
            2 => {
                let p1 = match (opcodes[position] / 100) % 10 {
                    0 => {
                        opcodes[opcodes[position + 1] as usize]
                    },
                    1 => {
                        opcodes[position + 1]
                    },
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let p2 = match (opcodes[position] / 1000) % 10 {
                    0 => {
                        opcodes[opcodes[position + 2] as usize]
                    },
                    1 => {
                        opcodes[position + 2]
                    },
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                let output = p1 * p2;
                let output_position = opcodes[position + 3] as usize;
                opcodes[output_position] = output;
                position += 4;
            },
            3 => {
                let output = 1;
                let output_position = opcodes[position + 1] as usize;
                opcodes[output_position] = output;
                position += 2;
            },
            4 => {
                let output = match (opcodes[position] / 100) % 10 {
                    0 => {
                        opcodes[opcodes[position + 1] as usize]
                    },
                    1 => {
                        opcodes[position + 1]
                    },
                    _ => panic!("Invalid mode at position {}", opcodes[position]),
                };
                println!("{}", output);
                position += 2;
            },
            99 => break,
            _ => panic!("Invalid opcode at position {} for {:?}", position, opcodes),
        };
    }

    opcodes
}

fn main() {
    let input = read_input();
    let opcodes = parse_opcodes(&input);
    get_final_state(opcodes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100243433() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("1002,4,3,4,33"))),
            vec![1002, 4, 3, 4, 99],
        );
    }

    #[test]
    fn test_1101100140() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("1101,100,-1,4,0"))),
            vec![1101, 100, -1, 4, 99],
        );
    }
}
