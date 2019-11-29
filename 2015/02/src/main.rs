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

pub fn present_wrapping_paper(input: &str) -> usize {
    let dimensions = input
        .split("x")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let l = dimensions[0];
    let w = dimensions[1];
    let h = dimensions[2];

    let sides: [usize; 3] = [(2 * l * w), (2 * w * h), (2 * h * l)];

    sides.iter().sum::<usize>() + (*sides.iter().min().unwrap() / 2)
}

fn calculate_wrapping_paper(input: String) -> usize {
    let mut total_wrapping_paper = 0;
    for line in input.lines() {
        total_wrapping_paper += present_wrapping_paper(line);
    }
    total_wrapping_paper
}

fn main() {
    let input = read_input();
    let total_wrapping_paper = calculate_wrapping_paper(input);
    print!("The elves will need {} square feet of wrapping paper.", format_number(total_wrapping_paper));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_3_4() {
        assert_eq!(present_wrapping_paper("2x3x4"), 58);
    }

    #[test]
    fn test_1_1_10() {
        assert_eq!(present_wrapping_paper("1x1x10"), 43);
    }
}
