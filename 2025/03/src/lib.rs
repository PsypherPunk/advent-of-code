fn get_joltage(bank: &[u8], digits: usize) -> Result<Vec<u8>, String> {
    if digits == 1 {
        let max = bank.iter().max().ok_or("bank is empty")?;
        return Ok(vec![*max]);
    }

    let digit = bank[..bank.len() - digits + 1]
        .iter()
        // .enumerate().max_by_key(|&(i, &b)| b)
        // …doesn't necessarily get the first occurrence (i.e. wrong `i`)
        // .enumerate().max_by(|(i1, &b1), (i2, &b2)| b1.cmp(&b2).then(i2.cmp(i1)))
        // …is slower
        .max()
        .ok_or("bank is empty")?;

    let mut result = vec![*digit];

    let i = bank
        .iter()
        .position(|&b| b == *digit)
        .ok_or("digit not found in bank")?
        + 1;

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

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let total_output_joltage = input
        .trim()
        .lines()
        .map(|line| {
            let joltage = get_joltage(line.as_bytes(), 12)?;
            Ok(joltage
                .iter()
                .fold(0, |acc, &b| acc * 10 + (b - b'0') as usize))
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(total_output_joltage.iter().sum())
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
        assert_eq!(Ok(3121910778619), get_part_two(INPUT));
    }
}
