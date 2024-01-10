//! Can be viewed as a
//! [binomial coefficient](https://en.wikipedia.org/wiki/Binomial_coefficient)
//! problem.
//!
//! Specifically,
//! [Pascal's Triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle) can
//! be used to simplify things.

pub fn get_part_one(input: &str) -> Result<isize, String> {
    let report = input
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|digit| digit.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()
        .map_err(|e| e.to_string())?;
    let num_values = report.first().ok_or(format!("bad input: {}", input))?.len() as isize;

    let mut coefficient = 1;
    let mut triangle = vec![coefficient];

    // TODO: as an iterator?
    for i in 0..num_values {
        coefficient = (coefficient * (i - num_values)) / (i + 1);
        triangle.push(coefficient);
    }

    let sum = report
        .iter()
        .map(|history| {
            history
                .iter()
                .enumerate()
                .map(|(i, value)| value * triangle[i])
                .sum::<isize>()
        })
        .sum::<isize>();

    Ok(sum.abs())
}

pub fn get_part_two(input: &str) -> Result<isize, String> {
    let report = input
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|digit| digit.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()
        .map_err(|e| e.to_string())?;
    let num_values = report.first().ok_or(format!("bad input: {}", input))?.len() as isize;

    let mut coefficient = 1;
    let mut triangle = vec![coefficient];

    // TODO: as an iterator?
    for i in 0..num_values {
        coefficient = (coefficient * (i - num_values)) / (i + 1);
        triangle.push(coefficient);
    }

    let sum = report
        .iter()
        .map(|history| {
            history
                .iter()
                .enumerate()
                .map(|(i, value)| value * triangle[i + 1])
                .sum::<isize>()
        })
        .sum::<isize>();

    Ok(sum.abs())
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
