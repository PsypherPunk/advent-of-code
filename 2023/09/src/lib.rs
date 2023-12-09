use std::collections::BTreeSet;

fn extrapolate(history: Vec<isize>) -> Result<isize, String> {
    let values = history.iter().copied().collect::<BTreeSet<_>>();

    match values.len() {
        1 => {
            let value = values
                .first()
                .ok_or(format!("couldn't extrapolate: {:?}", values))?;

            Ok(*value)
        }
        _ => {
            let next = history
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();
            let last = history
                .last()
                .ok_or(format!("no last number: {:?}", history))?;

            Ok(last + extrapolate(next)?)
        }
    }
}

pub fn get_part_one(input: &str) -> Result<isize, String> {
    let sum = input
        .trim()
        .lines()
        .map(|line| {
            let history = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<isize>().map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;

            extrapolate(history)
        })
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(sum)
}

pub fn get_part_two(_input: &str) -> Result<isize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(114), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
