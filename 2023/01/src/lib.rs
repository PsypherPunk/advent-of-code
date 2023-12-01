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

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let calibration_values = input
        .trim()
        .lines()
        .map(|line| {
            let mut positions = numbers
                .iter()
                .flat_map(|number| line.match_indices(number))
                .collect::<Vec<_>>();

            positions.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

            match (positions.first(), positions.last()) {
                (Some(&(_, a)), Some(&(_, b))) => {
                    let a = match a {
                        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            a.parse::<usize>().map_err(|_| "invalid digits".to_owned())
                        }
                        word => Ok(numbers
                            .iter()
                            .position(|&n| n == word)
                            .ok_or("invalid word".to_owned())?
                            - 8),
                    };
                    let b = match b {
                        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            b.parse::<usize>().map_err(|_| "invalid digits".to_owned())
                        }
                        word => Ok(numbers
                            .iter()
                            .position(|&n| n == word)
                            .ok_or("invalid word".to_owned())?
                            - 8),
                    };

                    Ok((a? * 10) + b?)
                }
                _ => Err("invalid line".to_owned()),
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
