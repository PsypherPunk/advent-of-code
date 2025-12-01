use std::str::FromStr;

#[derive(Debug)]
enum Rotation {
    Left(isize),
    Right(isize),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let dir = s.chars().next().ok_or_else(|| "empty line".to_string())?;
        let value: isize = s[1..]
            .parse()
            .map_err(|e| format!("invalid distance: {}", e))?;

        match dir {
            'L' => Ok(Rotation::Left(value)),
            'R' => Ok(Rotation::Right(value)),
            _ => Err(format!("invalid direction: {}", dir)),
        }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let rotations = input
        .lines()
        .map(Rotation::from_str)
        .collect::<Result<Vec<Rotation>, _>>()?;

    let count = rotations
        .iter()
        .scan(50, |state, rotation| {
            match rotation {
                Rotation::Left(distance) => *state = (((*state - distance) % 100) + 100) % 100,
                Rotation::Right(distance) => *state = (((*state + distance) % 100) + 100) % 100,
            };
            Some(*state)
        })
        .filter(|&position| position == 0)
        .count();

    Ok(count)
}

pub fn get_part_two(input: &str) -> Result<isize, String> {
    let rotations = input
        .lines()
        .map(Rotation::from_str)
        .collect::<Result<Vec<Rotation>, _>>()?;

    let count = rotations
        .iter()
        .scan(50, |state, rotation| {
            let inc = match rotation {
                Rotation::Left(distance) => {
                    let zero = if *state == 0 { 100 } else { *state };
                    let inc = if *distance >= zero {
                        1 + (*distance - zero) / 100
                    } else {
                        0
                    };

                    *state = (((*state - distance) % 100) + 100) % 100;

                    inc
                }
                Rotation::Right(dist) => {
                    let inc = (*state + *dist) / 100;
                    *state = (((*state + dist) % 100) + 100) % 100;

                    inc
                }
            };

            Some(inc)
        })
        .sum();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(3), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(6), get_part_two(INPUT));
    }
}
