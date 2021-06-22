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

fn parse_dimensions(input: &str) -> Vec<usize> {
    input
        .split('x')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn present_wrapping_paper(input: &str) -> usize {
    let dimensions = parse_dimensions(input);
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];

    let sides: [usize; 3] = [(2 * l * w), (2 * w * h), (2 * h * l)];

    sides.iter().sum::<usize>() + (*sides.iter().min().unwrap() / 2)
}

fn calculate_wrapping_paper(input: &str) -> usize {
    let mut total_wrapping_paper = 0;
    for line in input.lines() {
        total_wrapping_paper += present_wrapping_paper(line);
    }
    total_wrapping_paper
}

fn present_ribbon(input: &str) -> usize {
    let mut dimensions = parse_dimensions(input);
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];
    dimensions.sort_unstable();

    (2 * dimensions[0]) + (2 * dimensions[1]) + l * w * h
}

fn calculate_ribbon(input: &str) -> usize {
    let mut total_ribbon = 0;
    for line in input.lines() {
        total_ribbon += present_ribbon(line);
    }
    total_ribbon
}

fn main() {
    let input = read_input();
    let total_wrapping_paper = calculate_wrapping_paper(&input);
    let total_ribbon = calculate_ribbon(&input);
    println!(
        "…square feet of wrapping paper…? {}",
        format_number(total_wrapping_paper),
    );
    println!("…feet of ribbon…? {}", format_number(total_ribbon));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping_2_3_4() {
        assert_eq!(present_wrapping_paper("2x3x4"), 58);
    }

    #[test]
    fn test_wrapping_1_1_10() {
        assert_eq!(present_wrapping_paper("1x1x10"), 43);
    }

    #[test]
    fn test_ribbon_2_3_4() {
        assert_eq!(present_ribbon("2x3x4"), 34);
    }

    #[test]
    fn test_ribbon_1_1_10() {
        assert_eq!(present_ribbon("1x1x10"), 14);
    }
}
