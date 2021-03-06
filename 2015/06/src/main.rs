use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

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

fn get_lit_count(input: &str) -> usize {
    let mut grid: HashSet<(usize, usize)> = HashSet::new();

    let instruction =
        Regex::new(r"^(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in input.lines() {
        let captures = instruction.captures(line).unwrap();
        let y_start = &captures[3].parse::<usize>().unwrap();
        let y_end = &captures[5].parse::<usize>().unwrap();
        let x_start = &captures[2].parse::<usize>().unwrap();
        let x_end = &captures[4].parse::<usize>().unwrap();
        match &captures[1] {
            "turn on" => {
                for y in *y_start..=*y_end {
                    for x in *x_start..=*x_end {
                        grid.insert((x, y));
                    }
                }
            }
            "turn off" => {
                for y in *y_start..=*y_end {
                    for x in *x_start..=*x_end {
                        grid.remove(&(x, y));
                    }
                }
            }
            "toggle" => {
                for y in *y_start..=*y_end {
                    for x in *x_start..=*x_end {
                        if grid.contains(&(x, y)) {
                            grid.remove(&(x, y));
                        } else {
                            grid.insert((x, y));
                        }
                    }
                }
            }
            _ => panic!("Unknown instruction: {}", &captures[1]),
        };
    }

    grid.len()
}

fn get_brightness(input: String) -> isize {
    let mut grid: HashMap<(usize, usize), isize> = HashMap::new();

    let instruction =
        Regex::new(r"^(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in input.lines() {
        let captures = instruction.captures(line).unwrap();
        let y_start = &captures[3].parse::<usize>().unwrap();
        let y_end = &captures[5].parse::<usize>().unwrap();
        let x_start = &captures[2].parse::<usize>().unwrap();
        let x_end = &captures[4].parse::<usize>().unwrap();
        match &captures[1] {
            "turn on" => {
                for y in *y_start..=*y_end {
                    for x in *x_start..=*x_end {
                        let brightness = grid.entry((x, y)).or_insert(0);
                        *brightness += 1;
                    }
                }
            }
            "turn off" => {
                for y in *y_start..=*y_end {
                    for x in *x_start..=*x_end {
                        let brightness = grid.entry((x, y)).or_insert(1);
                        *brightness = (*brightness - 1).max(0);
                    }
                }
            }
            "toggle" => {
                for y in *y_start..=*y_end {
                    for x in *x_start..=*x_end {
                        let brightness = grid.entry((x, y)).or_insert(0);
                        *brightness += 2;
                    }
                }
            }
            _ => panic!("Unknown instruction: {}", &captures[1]),
        };
    }

    grid.values().sum()
}

fn main() {
    let input = read_input();
    let lit_count = get_lit_count(&input);
    println!("How many lights are lit? {}", lit_count);
    let total_brightness = get_brightness(input);
    println!("What is the total brightness? {}", total_brightness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_on_0_0_through_999_999() {
        assert_eq!(
            get_lit_count(&String::from("turn on 0,0 through 999,999")),
            1_000_000
        );
    }

    #[test]
    fn test_toggle_0_0_through_999_0() {
        assert_eq!(
            get_lit_count(&String::from("toggle 0,0 through 999,0")),
            1_000
        );
    }

    #[test]
    fn test_turn_off_499_499_through_500_500() {
        assert_eq!(
            get_lit_count(&String::from("turn off 499,499 through 500,500")),
            0
        );
    }

    #[test]
    fn test_turn_on_0_0_through_0_0() {
        assert_eq!(get_brightness(String::from("turn on 0,0 through 0,0")), 1);
    }

    #[test]
    fn test_toggle_0_0_through_999_999() {
        assert_eq!(
            get_brightness(String::from("toggle 0,0 through 999,999")),
            2_000_000
        );
    }
}
