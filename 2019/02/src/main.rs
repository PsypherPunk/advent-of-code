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

fn parse_opcodes(initial_state: &String) -> Vec<usize> {
    initial_state
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn get_final_state(mut opcodes: Vec<usize>) -> Vec<usize> {
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
}

fn find_noun_verb(input: String) -> Vec<usize> {
    let initial_state = parse_opcodes(&input);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut opcodes = initial_state.clone();
            opcodes[1] = noun;
            opcodes[2] = verb;
            let final_state = get_final_state(opcodes.clone());
            if final_state[0] == 19690720 {
                return opcodes.clone();
            }
        }
    }
    panic!("Could not find noun/verb.")
}

fn main() {
    let input = read_input();
    let mut opcodes = parse_opcodes(&input);
    opcodes[1] = 12;
    opcodes[2] = 2;
    let final_state = get_final_state(opcodes);
    println!("The value in position 0 is: {}", final_state[0]);

    let final_state = find_noun_verb(input);
    println!("{:?}", final_state);
    println!(
        "100 * noun + verb is {}",
        100 * final_state[1] + final_state[2]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100099() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("1,0,0,0,99"))),
            vec![2, 0, 0, 0, 99]
        );
    }

    #[test]
    fn test_230399() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("2,3,0,3,99"))),
            vec![2, 3, 0, 6, 99]
        );
    }

    #[test]
    fn test_2445990() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("2,4,4,5,99,0"))),
            vec![2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn test_11149956099() {
        assert_eq!(
            get_final_state(parse_opcodes(&String::from("1,1,1,4,99,5,6,0,99"))),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
