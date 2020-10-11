use std::fs;

fn scramble(instructions: &str, password: &str) -> String {
    let mut password = password.chars().collect::<Vec<char>>();

    for line in instructions.trim().lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "swap" => match words[1] {
                "position" => {
                    let a = words[2].parse::<usize>().unwrap();
                    let b = words[5].parse::<usize>().unwrap();
                    password.swap(a, b);
                }
                "letter" => {
                    let a = words[2].chars().next().unwrap();
                    let b = words[5].chars().next().unwrap();
                    let a = password.iter().position(|&c| c == a).unwrap();
                    let b = password.iter().position(|&c| c == b).unwrap();
                    password.swap(a, b);
                }
                _ => panic!(r#"Errr…¯\_(ツ)_/¯"#),
            },
            "rotate" => match words[1] {
                "left" => {
                    let rotation = words[2].parse::<usize>().unwrap();
                    password.rotate_left(rotation);
                }
                "right" => {
                    let rotation = words[2].parse::<usize>().unwrap();
                    password.rotate_right(rotation);
                }
                "based" => {
                    let a = words[6].chars().next().unwrap();
                    let position = password.iter().position(|&c| c == a).unwrap();
                    let rotation = if position >= 4 {
                        position + 2
                    } else {
                        position + 1
                    } % password.len();
                    password.rotate_right(rotation);
                }
                _ => panic!(r#"Errr…¯\_(ツ)_/¯"#),
            },
            "reverse" => {
                let a = words[2].parse::<usize>().unwrap();
                let b = words[4].parse::<usize>().unwrap();
                let start = password[0..a].to_owned();
                let mut middle = password[a..=b].to_owned();
                middle.reverse();
                let mut end = password[(b + 1)..].to_owned();

                password = start;
                password.append(&mut middle);
                password.append(&mut end);
            }
            "move" => {
                let a = words[2].parse::<usize>().unwrap();
                let b = words[5].parse::<usize>().unwrap();
                let ch = password.remove(a);
                password.insert(b, ch);
            }
            _ => panic!(r#"Errr…¯\_(ツ)_/¯"#),
        }
    }

    password.iter().collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the result of scrambling abcdefgh? {}",
        scramble(&input, "abcdefgh"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"#;

        assert_eq!("decab", scramble(&input, "abcde"));
    }
}
