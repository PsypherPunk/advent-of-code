use itertools::Itertools;

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let priorities = input
        .trim()
        .lines()
        .map(|line| {
            let length = line.len();
            let first = &line[..(length / 2)];
            let second = &line[(length / 2)..];

            let common = first
                .chars()
                .find(|c| second.contains(|a| a == *c))
                .ok_or_else(|| format!("no matching characters: {}, {}", first, second))?;

            let priority = match common.is_ascii_lowercase() {
                true => common as usize - 96,
                false => (common as usize - 64) + 26,
            };

            Ok::<usize, String>(priority)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(priorities)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let priorities = input
        .trim()
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(one, two, three)| {
            let common = one
                .chars()
                .find(|c| two.contains(|a| a == *c) && three.contains(|b| b == *c))
                .ok_or_else(|| format!("no matching characters: {}, {}, {}", one, two, three))?;

            let priority = match common.is_ascii_lowercase() {
                true => common as usize - 96,
                false => (common as usize - 64) + 26,
            };

            Ok::<usize, String>(priority)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(priorities)
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
        assert_eq!(Ok(157), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(70), get_part_two(INPUT));
    }
}
