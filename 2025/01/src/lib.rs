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

        if s.is_empty() {
            return Err("empty line".to_string());
        }

        let mut chars = s.chars();
        let dir = chars.next().ok_or_else(|| "empty line".to_string())?;
        let rest: String = chars.collect();
        let value: isize = rest
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
                Rotation::Left(dist) => *state = (((*state - dist) % 100) + 100) % 100,
                Rotation::Right(dist) => *state = (((*state + dist) % 100) + 100) % 100,
            };
            Some(*state)
        })
        .filter(|&position| position == 0)
        .count();

    Ok(count)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
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
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
