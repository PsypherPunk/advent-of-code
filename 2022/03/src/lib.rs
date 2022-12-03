pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let length = line.len();
            let first = &line[..(length / 2)];
            let second = &line[(length / 2)..];

            let common = first
                .chars()
                .find(|c| second.contains(|a| a == *c))
                .unwrap();

            match common.is_ascii_lowercase() {
                true => common as usize - 96,
                false => (common as usize - 64) + 26,
            }
        })
        .sum()
}

pub fn get_part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|group| {
            let common = group[0]
                .chars()
                .find(|c| group[1].contains(|a| a == *c) && group[2].contains(|b| b == *c))
                .unwrap();

            match common.is_ascii_lowercase() {
                true => common as usize - 96,
                false => (common as usize - 64) + 26,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(157, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(70, get_part_two(INPUT));
    }
}
