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

pub fn format_number(number: i32) -> String {
    let chars = number.to_string();
    let mut chars = chars.chars().rev();
    let mut reverse = (0..)
        .map(|_| chars.by_ref().take(3).collect::<String>())
        .take_while(|s| !s.is_empty())
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    reverse.reverse();

    reverse.join(",")
}

pub fn find_floor(instructions: String) -> i32 {
    let mut floor = 0;
    let mut position = 0;
    for line in instructions.lines() {
        for ch in line.chars() {
            position += 1;
            match ch {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!("Invalid character: {}", ch),
            }
            if floor == -1 {
                println!(
                    "Santa has entered the basement at character {}",
                    format_number(position)
                );
            }
        }
    }
    floor
}

fn main() {
    let input = read_input();
    let floor = find_floor(input);
    println!("The instructions take Santa to floor {}.", floor);
    format_number(1234567890);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(find_floor(String::from("(())")), 0);
        assert_eq!(find_floor(String::from("()()")), 0);
    }

    #[test]
    fn test_three() {
        assert_eq!(find_floor(String::from("(((")), 3);
        assert_eq!(find_floor(String::from("(()(()(")), 3);
        assert_eq!(find_floor(String::from("))(((((")), 3);
    }

    #[test]
    fn test_minus_one() {
        assert_eq!(find_floor(String::from("())")), -1);
        assert_eq!(find_floor(String::from("))(")), -1);
    }

    #[test]
    fn test_minus_three() {
        assert_eq!(find_floor(String::from(")))")), -3);
        assert_eq!(find_floor(String::from(")())())")), -3);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(123456), "123,456");
        assert_eq!(format_number(12345678), "12,345,678");
        assert_eq!(format_number(1234567890), "1,234,567,890");
    }
}
