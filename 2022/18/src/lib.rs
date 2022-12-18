#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::{HashSet, VecDeque};

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidMonkeyError(ParseError<LineCol>),
}

impl From<ParseError<LineCol>> for AdventOfCodeError {
    fn from(error: ParseError<LineCol>) -> Self {
        AdventOfCodeError::InvalidMonkeyError(error)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn adjacent(&self) -> [Self; 6] {
        [
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        ]
    }
}

peg::parser! {
    pub grammar scanner() for str {
        rule _() = [' ' | '\n']*

        rule number() -> isize
            = n:$("-"?['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule cube() -> Cube
            = x:number()
              ","
              y:number()
              ","
              z:number()
                { Cube { x, y, z } }

        pub rule cubes() -> HashSet<Cube>
            = cubes:cube() ++ _
             { cubes.into_iter().collect() }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let cubes = scanner::cubes(input.trim())?;

    let sides = cubes
        .iter()
        .map(|cube| {
            cube.adjacent()
                .iter()
                .filter(|adjacent| !cubes.contains(adjacent))
                .count()
        })
        .sum();

    Ok(sides)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let cubes = scanner::cubes(input.trim())?;

    let (mut maxx, mut maxy, mut maxz) = (0, 0, 0);
    cubes.iter().for_each(|cube| {
        (maxx, maxy, maxz) = (maxx.max(cube.x + 1), maxy.max(cube.y + 1), maxz.max(cube.z + 1))
    });
    let start = Cube {
        x: -1,
        y: -1,
        z: -1,
    };
    let end = Cube {
        x: maxx,
        y: maxy,
        z: maxz,
    };

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut surface_count = 0;

    queue.push_back(start);

    while let Some(cube) = queue.pop_front() {
        if !(seen.contains(&cube)
            || cube.x < start.x
            || cube.y < start.y
            || cube.z < start.z
            || cube.x > end.x
            || cube.y > end.y
            || cube.z > end.z)
        {
            seen.insert(cube);
            cube.adjacent().into_iter().for_each(|adjacent| {
                if cubes.contains(&adjacent) {
                    surface_count += 1;
                } else {
                    queue.push_back(adjacent);
                }
            });
        }
    }

    Ok(surface_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(64), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(58), get_part_two(INPUT));
    }
}
