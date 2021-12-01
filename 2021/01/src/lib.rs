pub fn get_larger_measurements(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

pub fn get_larger_sums(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|triple| triple.iter().sum::<usize>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(7, get_larger_measurements(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(5, get_larger_sums(INPUT));
    }
}
