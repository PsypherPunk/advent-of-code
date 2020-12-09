use std::fs;

use itertools::Itertools;

fn get_numbers(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn get_first_invalid_number(numbers: &[usize], window_size: usize) -> usize {
    numbers
        .iter()
        .skip(window_size)
        .zip(numbers.windows(window_size))
        .find(|(candidate, window)| {
            window
                .iter()
                .tuple_combinations()
                .find(|&(a, b)| a + b == **candidate)
                .is_none()
        })
        .map(|(invalid, _)| *invalid)
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let numbers = get_numbers(&input);
    let invalid_number = get_first_invalid_number(&numbers, 25);

    println!(
        "What is the first number that does not have this property? {}",
        invalid_number,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_part_one() {
        let numbers = get_numbers(&INPUT);

        assert_eq!(127, get_first_invalid_number(&numbers, 5))
    }
}
