fn get_elf_loads(input: &str) -> Result<Vec<Vec<usize>>, String> {
    input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().map_err(|e| e.to_string()))
                .collect::<Result<Vec<usize>, _>>()
        })
        .collect::<Result<Vec<Vec<usize>>, _>>()
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let elves = get_elf_loads(input)?;

    elves
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .max()
        .ok_or_else(|| "could not find maximum".to_string())
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let elves = get_elf_loads(input)?;

    let mut elves = elves
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .collect::<Vec<_>>();

    elves.sort();

    Ok(elves.iter().rev().take(3).sum())
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
        assert_eq!(Ok(24_000), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(45_000), get_part_two(INPUT));
    }
}
