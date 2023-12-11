use std::collections::BTreeSet;
use std::str::FromStr;

struct GiantImage {
    galaxies: Vec<(usize, usize)>,
    empty_columns: BTreeSet<usize>,
    empty_rows: BTreeSet<usize>,
}

impl FromStr for GiantImage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s
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
            .0;
        let height = galaxies
            .last()
            .ok_or(format!("invalid input: {:?}", galaxies))?
            .1;

        let empty_columns = (0..=width)
            .filter(|x| !galaxies.iter().any(|(gx, _)| gx == x))
            .collect::<BTreeSet<_>>();
        let empty_rows = (0..=height)
            .filter(|y| !galaxies.iter().any(|(_, gy)| gy == y))
            .collect::<BTreeSet<_>>();

        Ok(Self {
            galaxies,
            empty_columns,
            empty_rows,
        })
    }
}

impl GiantImage {
    fn get_lengths_sum(&self, expansion: usize) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                self.galaxies.iter().skip(i + 1).map(|b| {
                    a.0.abs_diff(b.0)
                        + a.1.abs_diff(b.1)
                        + self
                            .empty_columns
                            .iter()
                            .filter(|&x| *x > a.0.min(b.0) && *x < a.0.max(b.0))
                            .count()
                            * (expansion.saturating_sub(1))
                        + self
                            .empty_rows
                            .iter()
                            .filter(|&y| *y > a.1.min(b.1) && *y < a.1.max(b.1))
                            .count()
                            * (expansion.saturating_sub(1))
                })
            })
            .sum()
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let image = GiantImage::from_str(input)?;

    Ok(image.get_lengths_sum(2))
}

pub fn get_part_two(input: &str, expansion: usize) -> Result<usize, String> {
    let image = GiantImage::from_str(input)?;

    Ok(image.get_lengths_sum(expansion))
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
        assert_eq!(Ok(1030), get_part_two(INPUT, 10));
        assert_eq!(Ok(8410), get_part_two(INPUT, 100));
    }
}
