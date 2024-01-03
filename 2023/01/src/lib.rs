const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let calibration_values = input
        .trim()
        .lines()
        .map(|line| {
            let first = line.bytes().find(u8::is_ascii_digit);
            let last = line.bytes().rfind(u8::is_ascii_digit);

            match (first, last) {
                (Some(first), Some(last)) => {
                    Ok(((first as usize - 48) * 10) + (last as usize - 48))
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
            let first = line.char_indices().find_map(|(i, c)| {
                if c.is_ascii_digit() {
                    c.to_digit(10)
                } else {
                    NUMBERS.iter().enumerate().find_map(|(n, number)| {
                        match line[i..].starts_with(number) {
                            true => Some((n as u32) + 1),
                            false => None,
                        }
                    })
                }
            });
            let last = line.char_indices().rev().find_map(|(i, c)| {
                if c.is_ascii_digit() {
                    c.to_digit(10)
                } else {
                    NUMBERS.iter().enumerate().find_map(|(n, number)| {
                        match line[i..].starts_with(number) {
                            true => Some((n as u32) + 1),
                            false => None,
                        }
                    })
                }
            });

            match (first, last) {
                (Some(first), Some(last)) => Ok((first as usize * 10) + last as usize),
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
