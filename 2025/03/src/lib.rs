fn get_joltage(bank: &[u8], digits: usize) -> Result<Vec<u8>, String> {
    if digits == 1 {
        let max = bank
            .iter()
            .max()
            .ok_or_else(|| "bank is empty".to_string())?;
        return Ok(vec![*max]);
    }

    let digit = bank[..bank.len() - digits + 1]
        .iter()
        .max()
        .ok_or_else(|| "bank is empty".to_string())?;

    let i = bank
        .iter()
        .position(|&b| b == *digit)
        .ok_or_else(|| "digit not found in bank".to_string())?
        + 1;

    let mut result = vec![*digit];

    result.extend(get_joltage(&bank[i..], digits - 1)?);

    Ok(result)
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let total_output_joltage = input
        .trim()
        .lines()
        .map(|line| {
            let joltage = get_joltage(line.as_bytes(), 2)?;
            Ok(joltage
                .iter()
                .fold(0, |acc, &b| acc * 10 + (b - b'0') as usize))
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(total_output_joltage.iter().sum())
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(357), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
