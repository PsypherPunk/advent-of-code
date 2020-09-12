use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

struct Assembunny {
    registers: HashMap<String, isize>,
}

impl Assembunny {
    fn from_string(input: &str) -> Self {
        let mut registers = HashMap::new();

        let instructions = input.trim().lines().collect::<Vec<_>>();

        let mut offset = 0;

        while offset < instructions.len() {
            let ops = instructions[offset]
                .split_whitespace()
                .collect::<Vec<&str>>();
            match ops[0] {
                "cpy" => {
                    let x = match ops[1].parse::<isize>() {
                        Ok(x) => x,
                        Err(_) => *registers.entry(ops[1].to_string()).or_insert(0 as isize),
                    };
                    let y = registers.entry(ops[2].to_string()).or_insert(0 as isize);
                    *y = x;
                    offset += 1;
                }
                "inc" => {
                    let a = registers.entry(ops[1].to_string()).or_insert(0 as isize);
                    *a += 1;
                    offset += 1;
                }
                "dec" => {
                    let a = registers.entry(ops[1].to_string()).or_insert(0 as isize);
                    *a -= 1;
                    offset += 1;
                }
                "jnz" => {
                    let x = match ops[1].parse::<isize>() {
                        Ok(x) => x,
                        Err(_) => *registers.entry(ops[1].to_string()).or_insert(0 as isize),
                    };
                    let y = ops[2].parse::<isize>().unwrap();
                    match 0.cmp(&x) {
                        Ordering::Less => offset += y as usize,
                        Ordering::Greater => offset -= y as usize,
                        Ordering::Equal => offset += 1,
                    }
                }
                _ => panic!("Invalid instruction: {}", ops[0]),
            }
        }

        Assembunny { registers }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let assembunny = Assembunny::from_string(&input);

    println!(
        "…what value is left in register a? {}",
        assembunny.registers.get("a").unwrap(),
    );

    let assembunny = Assembunny::from_string(&("cpy 1 c\n".to_owned() + &input));

    println!(
        "…what value is now left in register a? {}",
        assembunny.registers.get("a").unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;

        let assembunny = Assembunny::from_string(&input);

        assert!(assembunny.registers.contains_key("a"));
        assert_eq!(42, *assembunny.registers.get("a").unwrap());
    }
}
