use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

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

fn read_instructions(input: &String) -> HashMap<&str, &str> {
    let instructions = input
        .lines()
        .map(|line| {
            let split = line.split(" -> ").collect::<Vec<&str>>();
            (split[1], split[0])
        })
        .collect::<Vec<(&str, &str)>>();

    let instructions: HashMap<&str, &str> = HashMap::from_iter(instructions);

    instructions
}

fn get_signal(wire: &str, instructions: &HashMap<&str, &str>) -> u16 {
    let signal = instructions.get(wire).unwrap();
    println!("{:?} -> {:?}", wire, signal);
    match *signal {
        and if and.contains("AND") => {
            let inputs = and.split(" ").collect::<Vec<&str>>();
            match inputs[0].parse::<u16>() {
                Ok(input) => input & get_signal(inputs[2], &instructions),
                Err(_) => {
                    get_signal(inputs[0], &instructions) & get_signal(inputs[2], &instructions)
                },
            }
        },
        or if or.contains("OR") => {
            let inputs = or.split(" ").collect::<Vec<&str>>();
            get_signal(inputs[0], &instructions) | get_signal(inputs[2], &instructions)
        },
        lshift if lshift.contains("LSHIFT") => {
            let inputs = lshift.split(" ").collect::<Vec<&str>>();
            get_signal(inputs[0], &instructions) << inputs[2].parse::<u16>().unwrap()
        },
        rshift if rshift.contains("RSHIFT") => {
            let inputs = rshift.split(" ").collect::<Vec<&str>>();
            get_signal(inputs[0], &instructions) >> inputs[2].parse::<u16>().unwrap()
        },
        not if not.contains("NOT") => {
            let inputs = not.split(" ").collect::<Vec<&str>>();
            !get_signal(inputs[1], &instructions)
        },
        number => match number.parse::<u16>() {
            Ok(n) => n,
            Err(_) => get_signal(number, &instructions),
        },
    }
}

fn main() {
    let input = read_input();
    let instructions = read_instructions(&input);
    let a = get_signal("a", &instructions);
    println!("…what signal is ultimately provided to wire a? {}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_on_0_0_through_999_999() {
        let instructions: String = String::from(
            r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#,
        );
        let instructions = read_instructions(&instructions);
        assert_eq!(get_signal("d", &instructions), 72);
        assert_eq!(get_signal("e", &instructions), 507);
        assert_eq!(get_signal("f", &instructions), 492);
        assert_eq!(get_signal("g", &instructions), 114);
        assert_eq!(get_signal("h", &instructions), 65412);
        assert_eq!(get_signal("i", &instructions), 65079);
        assert_eq!(get_signal("x", &instructions), 123);
        assert_eq!(get_signal("y", &instructions), 456);
    }
}
