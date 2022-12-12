#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidHeightMapError,
    TotallyLostError,
}

pub struct HeightMap {
    elevations: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn get_steps(&self, (x, y): (usize, usize), cmp: Ordering) -> Vec<(usize, usize)> {
        [
            (x, y.saturating_sub(1)),
            ((x + 1).min(self.width - 1), y),
            (x, (y + 1).min(self.height - 1)),
            (x.saturating_sub(1), y),
        ]
        .into_iter()
        .filter(|(dx, dy)| {
            (dx, dy) != (&x, &y)
                && (self.elevations[*dy][*dx].cmp(&self.elevations[y][x]) == cmp
                    || self.elevations[*dy][*dx].abs_diff(self.elevations[y][x]) <= 1)
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
    }
}

impl FromStr for HeightMap {
    type Err = AdventOfCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let mut width = None;
        let elevations: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                width = Some(line.len());
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((x, y));
                            0
                        }
                        'E' => {
                            end = Some((x, y));
                            26
                        }
                        elevation => (elevation as usize) - 97,
                    })
                    .collect()
            })
            .collect();
        let height = elevations.len();

        Ok(HeightMap {
            elevations,
            width: width.ok_or(AdventOfCodeError::InvalidHeightMapError)?,
            height,
            start: start.ok_or(AdventOfCodeError::InvalidHeightMapError)?,
            end: end.ok_or(AdventOfCodeError::InvalidHeightMapError)?,
        })
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let heightmap = HeightMap::from_str(input)?;

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((heightmap.start, 0));
    seen.insert(heightmap.start);
    let mut fewest_steps = None;

    while let Some((current, distance)) = queue.pop_front() {
        if current == heightmap.end {
            fewest_steps = Some(distance);
            break;
        }

        for step in heightmap.get_steps(current, Ordering::Less) {
            if !seen.contains(&step) {
                queue.push_back((step, distance + 1));
                seen.insert(step);
            }
        }
    }

    fewest_steps.ok_or(AdventOfCodeError::TotallyLostError)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let heightmap = HeightMap::from_str(input)?;

    let mut fewest_steps = None;

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((heightmap.end, 0));
    seen.insert(heightmap.end);

    while let Some(((x, y), distance)) = queue.pop_front() {
        if heightmap.elevations[y][x] == 0 {
            fewest_steps = Some(distance);
            break;
        }

        for step in heightmap.get_steps((x, y), Ordering::Greater) {
            if !seen.contains(&step) {
                queue.push_back((step, distance + 1));
                seen.insert(step);
            }
        }
    }

    fewest_steps.ok_or(AdventOfCodeError::TotallyLostError)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(31), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(29), get_part_two(INPUT));
    }
}
