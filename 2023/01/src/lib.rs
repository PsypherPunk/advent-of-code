pub fn get_part_one(input: &str) -> Result<usize, String> {
    let calibration_values = input
        .trim()
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            match (digits.first(), digits.last()) {
                (Some(first), Some(last)) => vec![first, last]
                    .into_iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .map_err(|_| "invalid digits".to_owned()),
                _ => Err("invalid line".to_owned()),
            }
        })
        .collect::<Result<Vec<usize>, String>>()?;

    Ok(calibration_values.iter().sum())
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(142), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
