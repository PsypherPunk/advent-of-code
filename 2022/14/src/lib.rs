use std::collections::HashSet;
use std::str::FromStr;
#[cfg(feature = "display")]
use std::{
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    thread,
    time::Duration,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    LackOfAbyssError,
}

pub struct Scan {
    rocks: HashSet<(usize, usize)>,
    sand: HashSet<(usize, usize)>,
    falling: (usize, usize),
}

#[cfg(feature = "display")]
impl Display for Scan {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let height = self.rocks.iter().max_by_key(|r| r.1).ok_or(FmtError)?.1;
        let left = self.rocks.iter().min_by_key(|r| r.0).ok_or(FmtError)?.0;
        let width = self.rocks.iter().max_by_key(|r| r.0).ok_or(FmtError)?.0;

        let output = (0..=height + 2)
            .map(|y| {
                (left - 2..=width + 2)
                    .map(|x| match self.rocks.get(&(x, y)) {
                        Some(_) => '#',
                        None => match self.sand.get(&(x, y)) {
                            Some(_) => 'o',
                            None => match self.falling == (x, y) {
                                true => 'o',
                                false => '.',
                            },
                        },
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
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

        Ok(Self {
            rocks,
            sand: HashSet::new(),
            falling: (500, 0),
        })
    }
}

impl Scan {
    fn is_blocked(&self, coordinate: &(usize, usize)) -> bool {
        self.rocks.contains(coordinate) || self.sand.contains(coordinate)
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut scan = Scan::from_str(input)?;
    #[cfg(feature = "display")]
    {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}\n", &scan);
        thread::sleep(Duration::from_millis(5_000));
    }

    let void = scan
        .rocks
        .iter()
        .map(|(_, y)| *y)
        .max()
        .ok_or(AdventOfCodeError::LackOfAbyssError)?;
    let mut units = 0;

    'outer: loop {
        scan.falling = (500, 0);
        loop {
            #[cfg(feature = "display")]
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{}\n", &scan);
                thread::sleep(Duration::from_millis(50));
            }

            if scan.falling.1 > void {
                break 'outer;
            }

            if !scan.is_blocked(&(scan.falling.0, scan.falling.1 + 1)) {
                scan.falling.1 += 1;
            } else if !scan.is_blocked(&(scan.falling.0 - 1, scan.falling.1 + 1)) {
                scan.falling = (scan.falling.0 - 1, scan.falling.1 + 1);
            } else if !scan.is_blocked(&(scan.falling.0 + 1, scan.falling.1 + 1)) {
                scan.falling = (scan.falling.0 + 1, scan.falling.1 + 1);
            } else {
                scan.sand.insert(scan.falling);
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
        if scan.sand.contains(&(500, 0)) {
            break;
        }

        scan.falling = (500, 0);

        loop {
            #[cfg(feature = "display")]
            {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{}\n", &scan);
                thread::sleep(Duration::from_millis(100));
            }

            if scan.falling.1 + 1 == floor {
                scan.rocks.insert(scan.falling);
                break;
            } else if !scan.is_blocked(&(scan.falling.0, scan.falling.1 + 1)) {
                scan.falling.1 += 1;
            } else if !scan.is_blocked(&(scan.falling.0 - 1, scan.falling.1 + 1)) {
                scan.falling = (scan.falling.0 - 1, scan.falling.1 + 1);
            } else if !scan.is_blocked(&(scan.falling.0 + 1, scan.falling.1 + 1)) {
                scan.falling = (scan.falling.0 + 1, scan.falling.1 + 1);
            } else {
                scan.sand.insert(scan.falling);
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
