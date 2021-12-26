use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::{thread, time};

#[derive(Eq, PartialEq)]
struct Region {
    locations: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations = s
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = locations[0].len();
        let height = locations.len();

        Ok(Self {
            locations,
            width,
            height,
        })
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let output = self
            .locations
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

impl Region {
    fn get_step(&self) -> Self {
        let mut east_cucumbers = self.locations.clone();

        for (y, row) in east_cucumbers.iter_mut().enumerate().take(self.height) {
            for x in 0..self.width {
                if self.locations[y][x] == '>' {
                    let east = (x + 1) % self.width;
                    if self.locations[y][east] == '.' {
                        row[east] = '>';
                        row[x] = '.';
                    }
                }
            }
        }

        let mut south_cucumbers = east_cucumbers.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if east_cucumbers[y][x] == 'v' {
                    let south = (y + 1) % self.height;
                    if east_cucumbers[south][x] == '.' {
                        south_cucumbers[south][x] = 'v';
                        south_cucumbers[y][x] = '.';
                    }
                }
            }
        }

        Self {
            locations: south_cucumbers,
            width: self.width,
            height: self.height,
        }
    }
}

pub fn get_part_one(input: &str, display: bool) -> usize {
    let mut region = Region::from_str(input).unwrap();

    if display {
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", region);
        thread::sleep(time::Duration::from_millis(100));
    }

    (1..)
        .find(|_| {
            let next = region.get_step();

            if display {
                print!("{}[2J", 27 as char);
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{}", next);
                thread::sleep(time::Duration::from_millis(100));
            }

            match region == next {
                true => true,
                false => {
                    region = next;
                    false
                }
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(58, get_part_one(INPUT, true));
    }
}
