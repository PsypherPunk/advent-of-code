use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use num_complex::Complex;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    ParseIntError(ParseIntError),
    InvalidInputError(String),
}

impl From<ParseIntError> for AdventOfCodeError {
    fn from(error: ParseIntError) -> Self {
        AdventOfCodeError::ParseIntError(error)
    }
}

struct Motion {
    direction: Complex<i32>,
    distance: i32,
}

struct Rope {
    knots: Vec<Complex<i32>>,
    seen: HashSet<Complex<i32>>,
}

impl Rope {
    fn from_knots(knots: usize) -> Self {
        Self {
            knots: vec![Complex::new(0, 0); knots],
            seen: HashSet::new(),
        }
    }

    fn move_knots(&mut self, motion: Motion) {
        for _ in 0..motion.distance {
            self.knots[0] += motion.direction;

            for knot in 1..self.knots.len() {
                let tug = self.knots[knot - 1] - self.knots[knot];

                if tug.re.abs() > 1 || tug.im.abs() > 1 {
                    self.knots[knot] += Complex::new(tug.re.signum(), tug.im.signum());
                }
            }
            self.seen.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

impl FromStr for Motion {
    type Err = AdventOfCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_once(' ').unwrap();
        let direction = match direction {
            "U" => Some(Complex::new(0, -1)),
            "L" => Some(Complex::new(-1, 0)),
            "D" => Some(Complex::new(0, 1)),
            "R" => Some(Complex::new(1, 0)),
            _ => None,
        }
        .ok_or_else(|| AdventOfCodeError::InvalidInputError(direction.to_owned()))?;
        let distance = distance.parse::<i32>()?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

fn get_tail_visits(input: &str, knots: usize) -> Result<usize, AdventOfCodeError> {
    let rope = input
        .trim()
        .lines()
        .try_fold(Rope::from_knots(knots), |mut rope, line| {
            let motion = Motion::from_str(line)?;

            rope.move_knots(motion);

            Ok::<Rope, AdventOfCodeError>(rope)
        })?;

    Ok(rope.seen.len())
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    get_tail_visits(input, 2)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    get_tail_visits(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    const LARGER_INPUT: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(13), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(1), get_part_two(INPUT));
    }

    #[test]
    fn test_part_two_larger() {
        assert_eq!(Ok(36), get_part_two(LARGER_INPUT));
    }
}
