#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidDigitError(char),
    LackOfTreesError,
}

struct TallTrees {
    height: usize,
    width: usize,
    trees: Vec<Vec<u32>>,
}

impl FromStr for TallTrees {
    type Err = AdventOfCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<_>> = s
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .ok_or(AdventOfCodeError::InvalidDigitError(c))
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        let height = trees.len();
        let width = trees
            .get(0)
            .ok_or(AdventOfCodeError::LackOfTreesError)?
            .len();

        Ok(Self {
            height,
            width,
            trees,
        })
    }
}

#[allow(unused)]
impl TallTrees {
    fn get_visible_count(&self) -> usize {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width).filter_map(move |x| match (x, y) {
                    (0, _) => Some((x, y)),
                    (_, 0) => Some((x, y)),
                    (right, _) if right == self.width - 1 => Some((x, y)),
                    (_, bottom) if bottom == self.height - 1 => Some((x, y)),
                    (x, y) => {
                        let from_left = (0..x).all(|left| self.trees[y][left] < self.trees[y][x]);
                        let from_right = (x + 1..self.width)
                            .rev()
                            .all(|right| self.trees[y][right] < self.trees[y][x]);
                        let from_top = (0..y).all(|top| self.trees[top][x] < self.trees[y][x]);
                        let from_bottom = (y + 1..self.height)
                            .rev()
                            .all(|bottom| self.trees[bottom][x] < self.trees[y][x]);

                        match from_left || from_right || from_top || from_bottom {
                            true => Some((x, y)),
                            false => None,
                        }
                    }
                })
            })
            .collect::<HashSet<_>>()
            .len()
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let trees = TallTrees::from_str(input)?;

    Ok(trees.get_visible_count())
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"30373
25512
65332
33549
35390
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
