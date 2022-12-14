use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    LackOfAbyssError,
}

struct Scan {
    rocks: HashSet<(usize, usize)>,
}

impl FromStr for Scan {
    type Err = AdventOfCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks = s
            .trim()
            .lines()
            .flat_map(|line| {
                line.split(" -> ")
                    .map(|coordinate| {
                        let (x, y) = coordinate.split_once(',').unwrap();
                        let x = x.parse::<usize>().unwrap();
                        let y = y.parse::<usize>().unwrap();

                        (x, y)
                    })
                    .tuple_windows()
                    .flat_map(|(start, end)| {
                        (start.0.min(end.0)..=start.0.max(end.0)).flat_map(move |x| {
                            (start.1.min(end.1)..=start.1.max(end.1)).map(move |y| (x, y))
                        })
                    })
            })
            .collect();

        Ok(Self { rocks })
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut scan = Scan::from_str(input)?;

    let void = scan
        .rocks
        .iter()
        .map(|(_, y)| *y)
        .max()
        .ok_or(AdventOfCodeError::LackOfAbyssError)?;
    let mut units = 0;

    'outer: loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 > void {
                break 'outer;
            }

            if !scan.rocks.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !scan.rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !scan.rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                scan.rocks.insert(sand);
                break;
            }
        }

        units += 1;
    }

    Ok(units)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut scan = Scan::from_str(input)?;

    let floor = scan
        .rocks
        .iter()
        .map(|(_, y)| *y)
        .max()
        .ok_or(AdventOfCodeError::LackOfAbyssError)?
        + 2;
    let mut units = 0;

    loop {
        if scan.rocks.contains(&(500, 0)) {
            break;
        }

        let mut sand = (500, 0);

        loop {
            if sand.1 + 1 == floor {
                scan.rocks.insert(sand);
                break;
            } else if !scan.rocks.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !scan.rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !scan.rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                scan.rocks.insert(sand);
                break;
            }
        }

        units += 1;
    }

    Ok(units)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(24), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(93), get_part_two(INPUT));
    }
}
