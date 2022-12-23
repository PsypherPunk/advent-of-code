#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::ops::Add;
use std::str::FromStr;

use itertools::{Itertools, MinMaxResult};

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    LackOfBoundariesError,
    HyperactiveElvesError,
}

#[derive(Debug)]
struct Scan {
    positions: HashSet<Position>,
}

impl Scan {
    const DIRECTIONS: [Position; 8] = [
        Position { x: 0, y: -1 },  // N
        Position { x: 1, y: -1 },  // NE
        Position { x: 1, y: 0 },   // E
        Position { x: 1, y: 1 },   // SE
        Position { x: 0, y: 1 },   // S
        Position { x: -1, y: 1 },  // SW
        Position { x: -1, y: 0 },  // W
        Position { x: -1, y: -1 }, // NW
    ];
    const CONSIDERED: [Position; 12] = [
        Position { x: 0, y: -1 },  // N
        Position { x: 1, y: -1 },  // NE
        Position { x: -1, y: -1 }, // NW
        Position { x: 0, y: 1 },   // S
        Position { x: 1, y: 1 },   // SE
        Position { x: -1, y: 1 },  // SW
        Position { x: -1, y: 0 },  // W
        Position { x: -1, y: -1 }, // NW
        Position { x: -1, y: 1 },  // SW
        Position { x: 1, y: 0 },   // E
        Position { x: 1, y: -1 },  // NE
        Position { x: 1, y: 1 },   // SE
    ];

    fn new() -> Self {
        Self {
            positions: HashSet::new(),
        }
    }

    fn minmax(&self) -> Result<(Position, Position), AdventOfCodeError> {
        match self.positions.iter().map(|p| p.x).minmax() {
            MinMaxResult::MinMax(minx, maxx) => match self.positions.iter().map(|p| p.y).minmax() {
                MinMaxResult::MinMax(miny, maxy) => {
                    Ok((Position::new(minx, miny), Position::new(maxx, maxy)))
                }
                _ => Err(AdventOfCodeError::LackOfBoundariesError),
            },
            _ => Err(AdventOfCodeError::LackOfBoundariesError),
        }
    }

    fn round(&self, number: usize) -> (Self, usize) {
        let direction = (number * 3) % 12;

        let (scan, moves) =
            (self.positions)
                .iter()
                .fold((Scan::new(), HashMap::new()), |mut acc, position| {
                    // "If no other Elves are in one of those eight positions, the Elf does not do anything…"
                    let other_elves = Scan::DIRECTIONS
                        .iter()
                        .find(|&movement| self.positions.contains(&(*position + *movement)));
                    if other_elves.is_none() {
                        acc.0.positions.insert(*position);
                        return acc;
                    }

                    for orthogonal in 0..4 {
                        let direction = (direction + (orthogonal * 3)) % Scan::CONSIDERED.len();
                        let adjacent_elf = (direction..direction + 3)
                            .find(|i| self.positions.contains(&(*position + Scan::CONSIDERED[*i])));
                        if adjacent_elf.is_none() {
                            let next = *position + Scan::CONSIDERED[direction];
                            acc.1.entry(next).or_insert_with(Vec::new).push(*position);
                            return acc;
                        }
                    }

                    acc.0.positions.insert(*position);

                    acc
                });

        moves.iter().fold(
            (scan, 0),
            |(mut acc, count), (position, moving)| match moving.len() {
                1 => {
                    acc.positions.insert(*position);
                    (acc, count + 1)
                }
                _ => {
                    acc.positions.extend(moving.iter());
                    (acc, count)
                }
            },
        )
    }
}

impl FromStr for Scan {
    type Err = AdventOfCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Position::new(x as isize, y as isize)),
                    _ => None,
                })
            })
            .collect();

        Ok(Self { positions })
    }
}

impl From<AdventOfCodeError> for FmtResult {
    fn from(_: AdventOfCodeError) -> Self {
        FmtResult::Err(FmtError)
    }
}

impl From<AdventOfCodeError> for FmtError {
    fn from(_: AdventOfCodeError) -> Self {
        FmtError
    }
}

impl Display for Scan {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let (min, max) = self.minmax()?;

        let output = (min.y..=max.y)
            .map(|y| {
                (min.x..=max.x)
                    .map(|x| match self.positions.contains(&Position::new(x, y)) {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut scan = Scan::from_str(input.trim())?;

    (0..10).for_each(|round| {
        (scan, _) = scan.round(round);
    });

    let (min, max) = scan.minmax()?;
    let empty = itertools::iproduct!(min.x..=max.x, min.y..=max.y)
        .filter(|(x, y)| !scan.positions.contains(&Position::new(*x, *y)))
        .count();

    Ok(empty)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut scan = Scan::from_str(input.trim())?;
    let mut count = 1;

    let round = (0..)
        .find(|round| {
            (scan, count) = scan.round(*round);

            count == 0
        })
        .ok_or(AdventOfCodeError::HyperactiveElvesError)?
        + 1;

    Ok(round)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(110), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(20), get_part_two(INPUT));
    }
}
