use std::num::ParseIntError;
use std::str::FromStr;

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Default)]
struct Position {
    horizontal: usize,
    depth: usize,
}

#[derive(Default)]
struct Course {
    position: Position,
    aim: usize,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, distance) = s
            .trim()
            .split_once(' ')
            .ok_or(format!("Invalid line: {}", s))?;

        let distance = distance.parse().map_err(|e: ParseIntError| e.to_string())?;

        match command {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            _ => Err(format!("Invalid direction: {}", command)),
        }
    }
}

pub fn get_part_one(input: &str) -> usize {
    let final_position = input
        .trim()
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .fold(Position::default(), |mut acc, command| {
            match command {
                Command::Forward(distance) => acc.horizontal += distance,
                Command::Down(distance) => acc.depth += distance,
                Command::Up(distance) => acc.depth -= distance,
            }
            acc
        });

    final_position.horizontal * final_position.depth
}

pub fn get_part_two(input: &str) -> usize {
    let final_course = input
        .trim()
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .fold(Course::default(), |mut acc, command| {
            match command {
                Command::Forward(units) => {
                    acc.position.horizontal += units;
                    acc.position.depth += acc.aim * units;
                }
                Command::Down(units) => acc.aim += units,
                Command::Up(units) => acc.aim -= units,
            }
            acc
        });

    final_course.position.horizontal * final_course.position.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(150, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(900, get_part_two(INPUT));
    }
}
