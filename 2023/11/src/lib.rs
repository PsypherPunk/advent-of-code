use std::collections::BTreeSet;

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let galaxies = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    let width = galaxies
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .ok_or(format!("invalid input: {:?}", galaxies))?
        .0
        + 1;
    let height = galaxies
        .last()
        .ok_or(format!("invalid input: {:?}", galaxies))?
        .1
        + 1;

    let empty_columns = (0..width)
        .filter(|x| !galaxies.iter().any(|coordinates| coordinates.0 == *x))
        .collect::<BTreeSet<_>>();
    let empty_rows = (0..height)
        .filter(|y| !galaxies.iter().any(|coordinates| coordinates.1 == *y))
        .collect::<BTreeSet<_>>();

    let lengths = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            galaxies.iter().skip(i + 1).map(|b| {
                a.0.abs_diff(b.0)
                    + a.1.abs_diff(b.1)
                    + empty_columns
                        .iter()
                        .filter(|&x| *x > a.0.min(b.0) && *x < a.0.max(b.0))
                        .count()
                    + empty_rows
                        .iter()
                        .filter(|&y| *y > a.1.min(b.1) && *y < a.1.max(b.1))
                        .count()
            })
        })
        .sum();

    Ok(lengths)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let galaxies = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    let width = galaxies
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .ok_or(format!("invalid input: {:?}", galaxies))?
        .0
        + 1;
    let height = galaxies
        .last()
        .ok_or(format!("invalid input: {:?}", galaxies))?
        .1
        + 1;

    let empty_columns = (0..width)
        .filter(|x| !galaxies.iter().any(|coordinates| coordinates.0 == *x))
        .collect::<BTreeSet<_>>();
    let empty_rows = (0..height)
        .filter(|y| !galaxies.iter().any(|coordinates| coordinates.1 == *y))
        .collect::<BTreeSet<_>>();

    let lengths = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            galaxies.iter().skip(i + 1).map(|b| {
                a.0.abs_diff(b.0)
                    + a.1.abs_diff(b.1)
                    + empty_columns
                        .iter()
                        .filter(|&x| *x > a.0.min(b.0) && *x < a.0.max(b.0))
                        .count()
                        * (1000000 - 1)
                    + empty_rows
                        .iter()
                        .filter(|&y| *y > a.1.min(b.1) && *y < a.1.max(b.1))
                        .count()
                        * (1000000 - 1)
            })
        })
        .sum();

    Ok(lengths)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(374), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(8410), get_part_two(INPUT));
    }
}
