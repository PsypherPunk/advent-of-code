const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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
                (Some(first), Some(last)) => {
                    Ok(((*first as usize - 48) * 10) + (*last as usize - 48))
                }
                _ => Err(format!("invalid line: {}", line)),
            }
        })
        .collect::<Result<Vec<usize>, String>>()?;

    Ok(calibration_values.iter().sum())
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let calibration_values = input
        .trim()
        .lines()
        .map(|line| {
            let digits = line
                .char_indices()
                .filter_map(|(position, c)| {
                    match NUMBERS.iter().enumerate().find_map(|(i, number)| {
                        match line[position..].starts_with(number) {
                            true => Some(i + 1),
                            false => None,
                        }
                    }) {
                        Some(number) => Some(number),
                        None => match c.is_ascii_digit() {
                            true => Some(c as usize - 48),
                            false => None,
                        },
                    }
                })
                .collect::<Vec<_>>();
            match (digits.first(), digits.last()) {
                (Some(first), Some(last)) => Ok((first * 10) + last),
                _ => Err(format!("invalid line: {}", line)),
            }
        })
        .collect::<Result<Vec<usize>, String>>()?;

    Ok(calibration_values.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;
    const INPUT_TWO: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(142), get_part_one(INPUT_ONE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(281), get_part_two(INPUT_TWO));
    }
}
