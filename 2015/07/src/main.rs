use std::collections::HashMap;
use std::fs;

fn read_instructions(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let split = line.trim().split(" -> ").collect::<Vec<&str>>();
            (split[1], split[0].trim().split(' ').collect::<Vec<&str>>())
        })
        .collect()
}

fn parse_instructions(wire: &str, instructions: &HashMap<&str, Vec<&str>>) -> u16 {
    let mut wires = HashMap::new();

    loop {
        if wires.contains_key(wire) {
            return *wires.get(wire).unwrap();
        }
        for (&dest, inputs) in instructions.iter() {
            if wires.contains_key(dest) {
                continue;
            }
            match inputs.len() {
                1 => match inputs[0].parse::<u16>() {
                    Ok(n) => {
                        wires.insert(dest, n);
                    }
                    Err(_) => {
                        if wires.contains_key(inputs[0]) {
                            wires.insert(dest, *wires.get(inputs[0]).unwrap());
                        }
                    }
                },
                _ => match inputs[0] {
                    "NOT" => {
                        if wires.contains_key(inputs[1]) {
                            wires.insert(dest, !*wires.get(inputs[1]).unwrap());
                        }
                    }
                    _ => match inputs[1] {
                        "AND" => match inputs[0].parse::<u16>() {
                            Ok(input) => {
                                if wires.contains_key(inputs[2]) {
                                    wires.insert(dest, input & *wires.get(inputs[2]).unwrap());
                                }
                            }
                            Err(_) => {
                                if wires.contains_key(inputs[0]) && wires.contains_key(inputs[2]) {
                                    wires.insert(
                                        dest,
                                        *wires.get(inputs[0]).unwrap()
                                            & *wires.get(inputs[2]).unwrap(),
                                    );
                                }
                            }
                        },
                        "OR" => {
                            if wires.contains_key(inputs[0]) && wires.contains_key(inputs[2]) {
                                wires.insert(
                                    dest,
                                    *wires.get(inputs[0]).unwrap() | *wires.get(inputs[2]).unwrap(),
                                );
                            }
                        }
                        "LSHIFT" => {
                            if wires.contains_key(inputs[0]) {
                                wires.insert(
                                    dest,
                                    *wires.get(inputs[0]).unwrap()
                                        << inputs[2].parse::<u16>().unwrap(),
                                );
                            }
                        }
                        "RSHIFT" => {
                            if wires.contains_key(inputs[0]) {
                                wires.insert(
                                    dest,
                                    *wires.get(inputs[0]).unwrap()
                                        >> inputs[2].parse::<u16>().unwrap(),
                                );
                            }
                        }
                        _ => panic!("Invalid operator: {}", inputs[1]),
                    },
                },
            }
        }
    }
}

/// Find the signal on `wire` recursively.
///
/// _Note_: this doesn't appear to work.
#[allow(dead_code)]
fn get_signal(wire: &str, instructions: &HashMap<&str, Vec<&str>>) -> u16 {
    let inputs = instructions.get(wire).unwrap();

    match inputs[0] {
        "NOT" => !get_signal(inputs[1], &instructions),
        _ => match inputs.len() {
            1 => match inputs[0].parse::<u16>() {
                Ok(n) => n,
                Err(_) => get_signal(inputs[0], &instructions),
            },
            _ => match inputs[1] {
                "AND" => match inputs[0].parse::<u16>() {
                    Ok(input) => input & get_signal(inputs[2], &instructions),
                    Err(_) => {
                        get_signal(inputs[0], &instructions) & get_signal(inputs[2], &instructions)
                    }
                },
                "OR" => get_signal(inputs[0], &instructions) | get_signal(inputs[2], &instructions),
                "LSHIFT" => {
                    get_signal(inputs[0], &instructions) << inputs[2].parse::<u16>().unwrap()
                }
                "RSHIFT" => {
                    get_signal(inputs[0], &instructions) >> inputs[2].parse::<u16>().unwrap()
                }
                _ => panic!("Invalid operator: {}", inputs[1]),
            },
        },
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let instructions = read_instructions(&input.trim());

    let a = parse_instructions("a", &instructions);
    println!("…what signal is ultimately provided to wire a? {}", a);

    let mut instructions = read_instructions(&input.trim());
    let input = &a.to_string()[..];
    instructions.insert("b", vec![input]);
    let a = parse_instructions("a", &instructions);
    println!("What new signal is ultimately provided to wire a? {}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_circuit() {
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

        assert_eq!(parse_instructions("d", &instructions), 72);
        assert_eq!(parse_instructions("e", &instructions), 507);
        assert_eq!(parse_instructions("f", &instructions), 492);
        assert_eq!(parse_instructions("g", &instructions), 114);
        assert_eq!(parse_instructions("h", &instructions), 65412);
        assert_eq!(parse_instructions("i", &instructions), 65079);
        assert_eq!(parse_instructions("x", &instructions), 123);
        assert_eq!(parse_instructions("y", &instructions), 456);
    }
}
