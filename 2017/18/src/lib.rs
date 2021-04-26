use std::collections::HashMap;

pub fn do_duet(input: &str) -> isize {
    let mut registers: HashMap<&str, isize> = HashMap::new();

    let instructions = input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut position = 0;
    let mut last_played_sound = None;

    loop {
        let instruction = &instructions[position];
        match instruction[0] {
            "snd" => {
                let a = registers.entry(instruction[1]).or_insert(0);
                last_played_sound = Some(*a);
                position += 1;
            }
            "set" => {
                let b = match instruction[2].parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => *registers.entry(instruction[2]).or_insert(0),
                };
                let a = registers.entry(instruction[1]).or_insert(0);
                *a = b;
                position += 1;
            }
            "add" => {
                let b = match instruction[2].parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => *registers.entry(instruction[2]).or_insert(0),
                };
                let a = registers.entry(instruction[1]).or_insert(0);
                *a += b;
                position += 1;
            }
            "mul" => {
                let b = match instruction[2].parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => *registers.entry(instruction[2]).or_insert(0),
                };
                let a = registers.entry(instruction[1]).or_insert(0);
                *a *= b;
                position += 1;
            }
            "mod" => {
                let b = match instruction[2].parse::<isize>() {
                    Ok(value) => value,
                    Err(_) => *registers.entry(instruction[2]).or_insert(0),
                };
                let a = registers.entry(instruction[1]).or_insert(0);
                *a %= b;
                position += 1;
            }
            "rcv" => {
                if let Some(rcv) = match (*registers.entry(instruction[1]).or_insert(0)).cmp(&0) {
                    std::cmp::Ordering::Equal => None,
                    _ => last_played_sound,
                } {
                    return rcv;
                }
                position += 1;
            }
            "jgz" => {
                position = match (*registers.entry(instruction[1]).or_insert(0)).cmp(&0) {
                    std::cmp::Ordering::Greater => {
                        let jump = match instruction[2].parse::<isize>() {
                            Ok(value) => value,
                            Err(_) => *registers.entry(instruction[2]).or_insert(0),
                        };
                        ((position as isize) + jump) as usize
                    }
                    _ => position + 1,
                };
            }
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"#;

    #[test]
    fn test_part_one() {
        assert_eq!(4, do_duet(&INPUT));
    }
}
