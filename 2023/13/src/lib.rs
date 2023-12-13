fn find_column_reflection(pattern: &[&[u8]], limit: usize) -> Option<usize> {
    (0..pattern[0].len() - 1).find(|&column| {
        let smudged = (0..=column.min(pattern[0].len() - column - 2))
            .map(|dc| {
                let left = column - dc;
                let right = column + 1 + dc;
                (0..pattern.len())
                    .filter(|&row| pattern[row][left] != pattern[row][right])
                    .count()
            })
            .sum::<usize>();

        smudged == limit
    })
}

fn find_row_reflection(pattern: &[&[u8]], limit: usize) -> Option<usize> {
    (0..pattern.len() - 1).find(|&row| {
        let smudged = (0..=row.min(pattern.len() - row - 2))
            .map(|dr| {
                let top = row - dr;
                let bottom = row + 1 + dr;
                (0..pattern[0].len())
                    .filter(|&column| pattern[top][column] != pattern[bottom][column])
                    .count()
            })
            .sum::<usize>();

        smudged == limit
    })
}

fn get_summary(patterns: Vec<Vec<&[u8]>>, limit: usize) -> Result<usize, String> {
    let notes = patterns
        .iter()
        .map(|pattern| {
            find_row_reflection(pattern, limit)
                .map(|row| (row + 1) * 100)
                .or_else(|| find_column_reflection(pattern, limit).map(|column| column + 1))
                .ok_or("could not find reflection".to_owned())
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(notes.iter().sum())
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.as_bytes())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    get_summary(patterns, 0)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.as_bytes())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    get_summary(patterns, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(405), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(400), get_part_two(INPUT));
    }
}
