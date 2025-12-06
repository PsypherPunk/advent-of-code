pub fn get_part_one(input: &str) -> Result<usize, String> {
    let count = input.lines().clone().count();

    let problems = input
        .lines()
        .take(count.saturating_sub(1))
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|digits| digits.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
        })
        .collect::<Result<Vec<Vec<usize>>, _>>()
        .map_err(|e| e.to_string())?;

    let grand_total = input
        .lines()
        .last()
        .ok_or("invalid input")?
        .split_ascii_whitespace()
        .enumerate()
        .map(|(position, op)| match op {
            "+" => Ok(problems.iter().map(|row| row[position]).sum::<usize>()),
            "*" => Ok(problems.iter().map(|row| row[position]).product::<usize>()),
            _ => Err("invalid operation"),
        })
        .collect::<Result<Vec<usize>, _>>()?
        .iter()
        .sum();

    Ok(grand_total)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(4277556), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
