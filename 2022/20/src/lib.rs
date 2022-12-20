use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    ParseIntError(ParseIntError),
    InvalidIndexError(usize),
    MisplacedZeroError,
}

impl From<ParseIntError> for AdventOfCodeError {
    fn from(error: ParseIntError) -> Self {
        AdventOfCodeError::ParseIntError(error)
    }
}

fn get_numbers(input: &str) -> Result<(Vec<usize>, Vec<isize>), AdventOfCodeError> {
    let (indices, numbers): (Vec<_>, Vec<Result<_, _>>) = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse::<isize>()))
        .unzip();

    Ok((indices, numbers.into_iter().collect::<Result<Vec<_>, _>>()?))
}

pub fn get_part_one(input: &str) -> Result<isize, AdventOfCodeError> {
    let (mut indices, numbers) = get_numbers(input)?;

    for i in 0..indices.len() {
        let index = indices
            .iter()
            .position(|p| *p == i)
            .ok_or(AdventOfCodeError::InvalidIndexError(i))?;

        let number = numbers[i];

        _ = indices.remove(index);

        let new_index = (index as isize + number).rem_euclid(numbers.len() as isize - 1);
        indices.insert(new_index as usize, i);
    }

    let numbers = indices
        .iter()
        .map(|index| numbers[*index])
        .collect::<Vec<_>>();

    let zero = numbers
        .iter()
        .position(|n| *n == 0)
        .ok_or(AdventOfCodeError::MisplacedZeroError)?;

    let coordinates = [1_000, 2_000, 3_000]
        .iter()
        .map(|position| numbers[(zero + *position) % numbers.len()])
        .sum::<isize>();

    Ok(coordinates)
}

pub fn get_part_two(input: &str) -> Result<isize, AdventOfCodeError> {
    let (mut indices, numbers) = get_numbers(input)?;

    let numbers = numbers.iter().map(|n| n * 811_589_153).collect::<Vec<_>>();

    for _ in 0..10 {
        for i in 0..indices.len() {
            let index = indices
                .iter()
                .position(|p| *p == i)
                .ok_or(AdventOfCodeError::InvalidIndexError(i))?;

            let number = numbers[i];

            _ = indices.remove(index);

            let new_index = (index as isize + number).rem_euclid(numbers.len() as isize - 1);
            indices.insert(new_index as usize, i);
        }
    }

    let numbers = indices
        .iter()
        .map(|index| numbers[*index])
        .collect::<Vec<_>>();

    let zero = numbers
        .iter()
        .position(|n| *n == 0)
        .ok_or(AdventOfCodeError::MisplacedZeroError)?;

    let coordinates = [1_000, 2_000, 3_000]
        .iter()
        .map(|position| numbers[(zero + *position) % numbers.len()])
        .sum::<isize>();

    Ok(coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1
2
-3
3
-2
0
4
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(3), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(1_623_178_306), get_part_two(INPUT));
    }
}
