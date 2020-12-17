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
}
