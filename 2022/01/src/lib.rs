pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn get_part_two(input: &str) -> usize {
    let mut elves = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    elves.sort();

    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(24_000, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(45_000, get_part_two(INPUT));
    }
}
