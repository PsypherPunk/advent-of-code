use std::collections::HashSet;
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

pub fn format_number(number: usize) -> String {
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

fn count_houses(input: &String) -> usize {
    let mut stops = vec![(0, 0)];
    for ch in input.chars() {
        let (x, y) = stops[0];
        stops.insert(
            0,
            match ch {
                '^' => (x, y + 1),
                '>' => (x + 1, y),
                'v' => (x, y - 1),
                '<' => (x - 1, y),
                _ => panic!("Invalid character: {}", ch),
            },
        );
    }
    let houses: HashSet<(i32, i32)> = HashSet::from_iter(stops);
    houses.len()
}

fn count_houses_with_robo_santa(input: &String) -> usize {
    let mut santa_stops = vec![(0, 0)];
    let mut robo_santa_stops = vec![(0, 0)];

    let (even, odd): (Vec<_>, Vec<_>) = input.chars().enumerate().partition(|(i, _)| (i % 2) == 0);

    let santa_instructions = even.iter().map(|(_, i)| i).collect::<Vec<&char>>();
    let robo_santa_instructions = odd.iter().map(|(_, i)| i).collect::<Vec<&char>>();

    for instruction in santa_instructions {
        let (x, y) = santa_stops[0];
        santa_stops.insert(
            0,
            match instruction {
                '^' => (x, y + 1),
                '>' => (x + 1, y),
                'v' => (x, y - 1),
                '<' => (x - 1, y),
                _ => panic!("Invalid character: {}", instruction),
            },
        );
    }

    for instruction in robo_santa_instructions {
        let (x, y) = robo_santa_stops[0];
        robo_santa_stops.insert(
            0,
            match instruction {
                '^' => (x, y + 1),
                '>' => (x + 1, y),
                'v' => (x, y - 1),
                '<' => (x - 1, y),
                _ => panic!("Invalid character: {}", instruction),
            },
        );
    }

    santa_stops.append(&mut robo_santa_stops);
    let houses: HashSet<(i32, i32)> = HashSet::from_iter(santa_stops);
    houses.len()
}

fn main() {
    let input = read_input();
    println!(
        "{} houses receive at least one present.",
        format_number(count_houses(&input))
    );
    println!(
        "This year, {} houses receive at least one present.",
        format_number(count_houses_with_robo_santa(&input))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(count_houses(String::from(">")), 2);
        assert_eq!(count_houses(String::from("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_four() {
        assert_eq!(count_houses(String::from("^>v<")), 4);
    }

    #[test]
    fn test_robo_three() {
        assert_eq!(count_houses_with_robo_santa(String::from("^v")), 3);
        assert_eq!(count_houses_with_robo_santa(String::from("^>v<")), 3);
    }

    #[test]
    fn test_robo_eleven() {
        assert_eq!(count_houses_with_robo_santa(String::from("^v^v^v^v^v")), 11);
    }
}
