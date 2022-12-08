#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidDigitError(char),
    LackOfTreesError,
    NotSoScenicError,
}

pub struct TallTrees {
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

impl TallTrees {
    fn is_visible_from_left(&self, (x, y): (usize, usize)) -> bool {
        (0..x).all(|left| self.trees[y][left] < self.trees[y][x])
    }

    fn is_visible_from_right(&self, (x, y): (usize, usize)) -> bool {
        (x + 1..self.width)
            .rev()
            .all(|right| self.trees[y][right] < self.trees[y][x])
    }

    fn is_visible_from_top(&self, (x, y): (usize, usize)) -> bool {
        (0..y).all(|top| self.trees[top][x] < self.trees[y][x])
    }

    fn is_visible_from_bottom(&self, (x, y): (usize, usize)) -> bool {
        (y + 1..self.height)
            .rev()
            .all(|bottom| self.trees[bottom][x] < self.trees[y][x])
    }

    fn get_visible_count(&self) -> usize {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width).filter_map(move |x| match (x, y) {
                    (0, _) => Some((x, y)),
                    (_, 0) => Some((x, y)),
                    (right, _) if right == self.width - 1 => Some((x, y)),
                    (_, bottom) if bottom == self.height - 1 => Some((x, y)),
                    (x, y) => {
                        match self.is_visible_from_top((x, y))
                            || self.is_visible_from_left((x, y))
                            || self.is_visible_from_right((x, y))
                            || self.is_visible_from_bottom((x, y))
                        {
                            true => Some((x, y)),
                            false => None,
                        }
                    }
                })
            })
            .collect::<HashSet<_>>()
            .len()
    }

    fn get_best_scenic_score(&self) -> Result<usize, AdventOfCodeError> {
        (1..self.height - 1)
            .flat_map(|y| {
                (1..self.width - 1)
                    .map(move |x| {
                        let up = {
                            let stop = (0..y)
                                .rev()
                                .map(|dy| self.trees[dy][x])
                                .enumerate()
                                .find(|(_, tree)| *tree >= self.trees[y][x]);
                            match stop {
                                Some((distance, _)) => distance + 1,
                                _ => y,
                            }
                        };
                        let down = {
                            let stop = (y + 1..self.height)
                                .map(|dy| self.trees[dy][x])
                                .enumerate()
                                .find(|(_, tree)| *tree >= self.trees[y][x]);
                            match stop {
                                Some((distance, _)) => distance + 1,
                                _ => (self.height - 1) - y,
                            }
                        };
                        let right = {
                            let stop = (x + 1..self.width)
                                .map(|dx| self.trees[y][dx])
                                .enumerate()
                                .find(|(_, tree)| *tree >= self.trees[y][x]);
                            match stop {
                                Some((distance, _)) => distance + 1,
                                _ => (self.width - 1) - x,
                            }
                        };
                        let left = {
                            let stop = (0..x)
                                .rev()
                                .map(|dx| self.trees[y][dx])
                                .enumerate()
                                .find(|(_, tree)| *tree >= self.trees[y][x]);
                            match stop {
                                Some((distance, _)) => distance + 1,
                                _ => x,
                            }
                        };

                        up * down * left * right
                    })
                    .max()
            })
            .max()
            .ok_or(AdventOfCodeError::LackOfTreesError)
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let trees = TallTrees::from_str(input)?;

    Ok(trees.get_visible_count())
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let trees = TallTrees::from_str(input)?;

    trees.get_best_scenic_score()
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
        assert_eq!(Ok(8), get_part_two(INPUT));
    }
}
