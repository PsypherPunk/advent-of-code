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

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let columns = input.lines().clone().count();
    let rows = input.lines().next().ok_or("invalid input")?.len();

    let problems = (0..rows)
        .rev()
        .map(|row| {
            (0..columns)
                .map(|column| input.lines().nth(column)?.as_bytes().get(row))
                .collect::<Option<Vec<&u8>>>()
        })
        .collect::<Option<Vec<Vec<&u8>>>>()
        .ok_or("invalid input")?;

    let grand_total = problems
        .iter()
        .filter(|line| !line.iter().all(|&b| *b == b' '))
        .scan(Vec::new(), |numbers, line| {
            let number = line
                .iter()
                .take(line.len().saturating_sub(1))
                .fold(0usize, |acc, &b| {
                    if b.is_ascii_digit() {
                        acc * 10 + (b - b'0') as usize
                    } else {
                        acc
                    }
                });

            numbers.push(number);

            match line.last() {
                Some(b'+') => {
                    let sum = numbers.iter().sum::<usize>();
                    numbers.clear();
                    Some(sum)
                }
                Some(b'*') => {
                    let product = numbers.iter().product();
                    numbers.clear();
                    Some(product)
                }
                _ => Some(0),
            }
        })
        .sum();

    Ok(grand_total)
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
        assert_eq!(Ok(3263827), get_part_two(INPUT));
    }
}
