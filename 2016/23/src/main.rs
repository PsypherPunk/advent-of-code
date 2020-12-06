use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

struct Assembunny {
    registers: HashMap<String, isize>,
}

impl Assembunny {
    fn from_string(input: &str, a: isize) -> Self {
        let mut registers = HashMap::new();
        registers.insert("a".to_string(), a);

        let mut instructions = input
            .trim()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let mut offset: isize = 0;

        while (offset as usize) < instructions.len() {
            let ops = instructions[offset as usize]
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
                    let y = match ops[2].parse::<isize>() {
                        Ok(y) => y,
                        Err(_) => *registers.entry(ops[2].to_string()).or_insert(0 as isize),
                    };
                    match 0.cmp(&x) {
                        Ordering::Less => offset += y,
                        Ordering::Greater => offset -= y,
                        Ordering::Equal => offset += 1,
                    }
                }
                "tgl" => {
                    let a = registers.get(&ops[1].to_string()).unwrap();
                    let target = offset + *a;
                    if target < 0 || (target as usize) >= instructions.len() {
                        offset += 1;
                        continue;
                    }
                    let toggled = instructions[target as usize]
                        .split_whitespace()
                        .collect::<Vec<&str>>();
                    let new = match toggled.len() {
                        2 => match toggled[0] {
                            "inc" => "dec",
                            _ => "inc",
                        },
                        3 => match toggled[0] {
                            "jnz" => "cpy",
                            _ => "jnz",
                        },
                        _ => panic!(),
                    };
                    let mut new_instruction = vec![new];
                    new_instruction.extend_from_slice(&toggled[1..]);
                    instructions[target as usize] = new_instruction.join(" ");
                    offset += 1;
                }
                _ => panic!("Invalid instruction: {}", ops[0]),
            }
        }

        Assembunny { registers }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let assembunny = Assembunny::from_string(&input, 7);

    println!(
        "What value should be sent to the safe? {}",
        assembunny.registers.get("a").unwrap(),
    );

    let assembunny = Assembunny::from_string(&input, 12);

    println!(
        "What value should be sent to the safe? {}",
        assembunny.registers.get("a").unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a"#;

        let assembunny = Assembunny::from_string(&input, 0);

        assert_eq!(3, *assembunny.registers.get("a").unwrap());
    }
}
