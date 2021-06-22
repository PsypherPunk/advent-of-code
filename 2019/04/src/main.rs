use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

use itertools::Itertools;

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

fn get_adjacent_duplicates(password: usize) -> Vec<String> {
    password
        .to_string()
        .chars()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect::<Vec<String>>()
}

fn has_two_adjacent_digits_the_same(password: usize) -> bool {
    let duplicates = get_adjacent_duplicates(password);

    duplicates.iter().filter(|g| g.len() > 1).count() > 0
}

fn has_exactly_two_adjacent_digits_the_same(password: usize) -> bool {
    let duplicates = get_adjacent_duplicates(password);

    duplicates.iter().filter(|g| g.len() == 2).count() > 0
}

fn has_non_decreasing_digits(password: usize) -> bool {
    let mut password_string = password.to_string().chars().collect::<Vec<char>>();
    password_string.sort_unstable();

    String::from_iter(password_string) == password.to_string()
}

fn get_possible_password_count(input: &str) -> usize {
    let inputs = input.trim().split('-').collect::<Vec<&str>>();

    let start = inputs[0].parse::<usize>().unwrap();
    let end = inputs[1].parse::<usize>().unwrap();

    let mut possibility_count = 0;
    for possibility in start..=end {
        if has_non_decreasing_digits(possibility) && has_two_adjacent_digits_the_same(possibility) {
            possibility_count += 1;
        }
    }
    possibility_count
}

fn get_better_possible_password_count(input: String) -> usize {
    let inputs = input.trim().split('-').collect::<Vec<&str>>();

    let start = inputs[0].parse::<usize>().unwrap();
    let end = inputs[1].parse::<usize>().unwrap();

    let mut possibility_count = 0;
    for possibility in start..=end {
        if has_non_decreasing_digits(possibility)
            && has_exactly_two_adjacent_digits_the_same(possibility)
        {
            possibility_count += 1;
        }
    }
    possibility_count
}

fn main() {
    let input = read_input();
    let possible_password_count = get_possible_password_count(&input);
    println!("How many different passwords…? {}", possible_password_count);
    let better_possible_password_count = get_better_possible_password_count(input);
    println!(
        "How many different passwords…? {}",
        better_possible_password_count,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111111() {
        assert_eq!(
            get_possible_password_count(&String::from("111111-111111")),
            1
        );
    }

    #[test]
    fn test_223450() {
        assert_eq!(
            get_possible_password_count(&String::from("223450-223450")),
            0
        );
    }

    #[test]
    fn test_123789() {
        assert_eq!(
            get_possible_password_count(&String::from("123789-123789")),
            0
        );
    }

    #[test]
    fn test_112233() {
        assert_eq!(
            get_better_possible_password_count(String::from("112233-112233")),
            1,
        );
    }

    #[test]
    fn test_123444() {
        assert_eq!(
            get_better_possible_password_count(String::from("123444-123444")),
            0,
        );
    }

    #[test]
    fn test_111122() {
        assert_eq!(
            get_better_possible_password_count(String::from("111122-111122")),
            1,
        );
    }
}
