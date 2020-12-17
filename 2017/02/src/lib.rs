pub fn get_evenly_divisible_values(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut numbers = line
                .trim()
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            numbers.sort_unstable();

            let (a, b) = numbers
                .iter()
                .enumerate()
                .flat_map(|(i, a)| {
                    numbers
                        .iter()
                        .skip(i + 1)
                        .map(move |b| (*a, *b))
                })
                .find(|(a, b)| *b % *a == 0)
                .unwrap();
            b / a
        })
        .sum()
}

pub fn get_checksum(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let numbers = line
                .trim()
                .split_whitespace()
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let min = numbers.iter().min().unwrap();
            let max = numbers.iter().max().unwrap();
            max - min
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"5 1 9 5
7 5 3
2 4 6 8"#;

        assert_eq!(18, get_checksum(&input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"5 9 2 8
9 4 7 3
3 8 6 5"#;

        assert_eq!(9, get_evenly_divisible_values(&input));
    }
}
