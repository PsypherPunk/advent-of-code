use itertools::Itertools;

pub fn get_numbers(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn get_first_invalid_number(numbers: &[usize], window_size: usize) -> usize {
    numbers
        .iter()
        .skip(window_size)
        .zip(numbers.windows(window_size))
        .find(|&(candidate, window)| {
            window
                .iter()
                .tuple_combinations()
                .find(|&(a, b)| a + b == *candidate)
                .is_none()
        })
        .map(|(invalid, _)| *invalid)
        .unwrap()
}

pub fn get_encryption_weakness(numbers: &[usize], invalid_number: usize) -> usize {
    let invalid_number_position = numbers
        .iter()
        .find(|&number| *number == invalid_number)
        .unwrap();

    let contiguous_set = (2..*invalid_number_position)
        .map(|window_size| {
            numbers
                .windows(window_size)
                .find(|contiguous_set| contiguous_set.iter().sum::<usize>() == invalid_number)
        })
        .find_map(|contiguous_set| contiguous_set)
        .unwrap();

    contiguous_set.iter().min().unwrap() + contiguous_set.iter().max().unwrap()
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

    #[test]
    fn test_part_two() {
        let numbers = get_numbers(&INPUT);
        let invalid_number = get_first_invalid_number(&numbers, 5);

        assert_eq!(62, get_encryption_weakness(&numbers, invalid_number));
    }
}
