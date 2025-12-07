use std::collections::HashSet;

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut lines = input.lines();

    let start = lines
        .next()
        .ok_or("invalid input")?
        .find('S')
        .ok_or("No starting point found".to_string())?;

    let total_splits = lines
        .scan(HashSet::from([start]), |beams, line| {
            let mut count = 0;

            *beams = beams
                .iter()
                .flat_map(|&position| match line.chars().nth(position) {
                    Some('^') => {
                        count += 1;
                        vec![position - 1, position + 1]
                    }
                    _ => vec![position],
                })
                .collect::<HashSet<_>>();

            Some(count)
        })
        .sum();

    Ok(total_splits)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(21), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
