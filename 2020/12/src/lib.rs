use std::str::FromStr;

use num::complex::Complex;

enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

struct Navigation {
    instructions: Vec<Action>,
}

pub struct Ferry {
    facing: Complex<i32>,
    position: Complex<i32>,
    bearing: Complex<i32>,
    navigation: Navigation,
    waypoint: Complex<i32>,
}

impl FromStr for Ferry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(|line| {
                let (action, value) = line.split_at(1);
                let value = value.parse().unwrap();
                match action.chars().next().unwrap() {
                    'N' => Action::North(value),
                    'S' => Action::South(value),
                    'E' => Action::East(value),
                    'W' => Action::West(value),
                    'L' => Action::Left(value),
                    'R' => Action::Right(value),
                    'F' => Action::Forward(value),
                    _ => panic!("Invalid char: {}", line),
                }
            })
            .collect();

        Ok(Self {
            facing: Complex::i(),
            position: Complex::new(0, 0),
            bearing: Complex::new(0, 0),
            navigation: Navigation { instructions },
            waypoint: Complex::new(1, 10),
        })
    }
}

impl Ferry {
    pub fn navigate(&mut self) -> Complex<i32> {
        (0..self.navigation.instructions.len()).for_each(|i| {
            let action = &self.navigation.instructions[i];
            match action {
                Action::North(value) => {
                    self.position += value;
                    self.waypoint += value;
                }
                Action::South(value) => {
                    self.position -= value;
                    self.waypoint -= value;
                }
                Action::East(value) => {
                    self.position.im += value;
                    self.waypoint.im += value;
                }
                Action::West(value) => {
                    self.position.im -= value;
                    self.waypoint.im -= value;
                }
                Action::Left(value) => {
                    self.facing *= Complex::i().powi(-value / 90);
                    self.waypoint *= Complex::i().powi(-value / 90);
                }
                Action::Right(value) => {
                    self.facing *= Complex::i().powi(value / 90);
                    self.waypoint *= Complex::i().powi(value / 90);
                }
                Action::Forward(value) => {
                    self.position += self.facing * value;
                    self.bearing += self.waypoint * value;
                }
            }
        });

        self.position
    }

    pub fn get_position_manhattan_distance(&self) -> i32 {
        self.position.im.abs() + self.position.re.abs()
    }

    pub fn get_bearing_manhattan_distance(&self) -> i32 {
        self.bearing.im.abs() + self.bearing.re.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn test_part_one() {
        let mut ferry = Ferry::from_str(&INPUT).unwrap();

        ferry.navigate();

        assert_eq!(25, ferry.position.re.abs() + ferry.position.im.abs());
    }

    #[test]
    fn test_part_two() {
        let mut ferry = Ferry::from_str(&INPUT).unwrap();

        ferry.navigate();

        assert_eq!(286, ferry.bearing.re.abs() + ferry.bearing.im.abs());
    }
}
