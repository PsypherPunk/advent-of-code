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

fn parse_opcodes(initial_state: String) -> Vec<usize> {
    initial_state
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn get_final_state(initial_state: String) -> String {
    let mut opcodes = parse_opcodes(initial_state);

    let mut position = 0;

    while position < opcodes.len() {
        let output = match opcodes[position] {
            1 => opcodes[opcodes[position + 1]] + opcodes[opcodes[position + 2]],
            2 => opcodes[opcodes[position + 1]] * opcodes[opcodes[position + 2]],
            99 => break,
            _ => panic!("Invalid opcode at position {}", position),
        };
        let output_position = opcodes[position + 3];
        opcodes[output_position] = output;
        position += 4;
    }

    opcodes
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn main() {
    let input = read_input();
    let mut opcodes = parse_opcodes(input);
    opcodes[1] = 12;
    opcodes[2] = 2;
    let initial_state = opcodes
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let final_state = get_final_state(initial_state);
    let final_state = final_state
        .trim()
        .split(",")
        .collect::<Vec<&str>>();
    println!("The value in position 0 is: {}", final_state[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100099() {
        assert_eq!(get_final_state(String::from("1,0,0,0,99")), "2,0,0,0,99");
    }

    #[test]
    fn test_230399() {
        assert_eq!(get_final_state(String::from("2,3,0,3,99")), "2,3,0,6,99");
    }

    #[test]
    fn test_2445990() {
        assert_eq!(get_final_state(String::from("2,4,4,5,99,0")), "2,4,4,5,99,9801");
    }

    #[test]
    fn test_11149956099() {
        assert_eq!(get_final_state(String::from("1,1,1,4,99,5,6,0,99")), "30,1,1,4,2,5,6,0,99");
    }
}
