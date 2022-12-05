#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidInputError,
    ParseIntError(ParseIntError),
    UnexpectedStackError(usize),
    EmptyStackError(usize),
}

impl From<ParseIntError> for AdventOfCodeError {
    fn from(error: ParseIntError) -> Self {
        AdventOfCodeError::ParseIntError(error)
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = AdventOfCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

        Ok(Move {
            count: parts[1].parse()?,
            from: parts[3]
                .chars()
                .next()
                .ok_or(AdventOfCodeError::InvalidInputError)?
                .to_digit(10)
                .ok_or(AdventOfCodeError::InvalidInputError)? as usize,
            to: parts[5]
                .chars()
                .next()
                .ok_or(AdventOfCodeError::InvalidInputError)?
                .to_digit(10)
                .ok_or(AdventOfCodeError::InvalidInputError)? as usize,
        })
    }
}

fn get_moves(input: &str) -> Result<Vec<Move>, AdventOfCodeError> {
    input.lines().map(Move::from_str).collect()
}

fn transpose<T: Clone + Copy>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::with_capacity(original.len()); original[0].len()];
    for row in original {
        for i in 0..row.len() {
            transposed[i].push(row[i]);
        }
    }

    transposed
}

fn get_cargo(input: &str) -> Result<Vec<VecDeque<char>>, AdventOfCodeError> {
    let rows = input
        .lines()
        .rev()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = transpose(rows);

    let cargo = rows
        .into_iter()
        .filter(|row| row[0] != ' ')
        .map(|row| row.into_iter().skip(1).filter(|c| *c != ' ').collect())
        .collect();

    Ok(cargo)
}

pub fn get_part_one(input: &str) -> Result<String, AdventOfCodeError> {
    let (cargo, moves) = input
        .trim_end()
        .split_once("\n\n")
        .ok_or(AdventOfCodeError::InvalidInputError)?;

    let moves = get_moves(moves)?;
    let mut cargo = get_cargo(cargo)?;

    for move_ in moves {
        let from_stack = &mut cargo[move_.from - 1];

        let mut crates = from_stack.split_off(from_stack.len() - move_.count);
        crates.make_contiguous().reverse();

        let to_stack = &mut cargo[move_.to - 1];
        to_stack.append(&mut crates);
    }

    let top = cargo
        .iter()
        .enumerate()
        .map(|(i, stack)| {
            stack
                .back()
                .ok_or(AdventOfCodeError::EmptyStackError(i + 1))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect();

    Ok(top)
}

pub fn get_part_two(input: &str) -> Result<String, AdventOfCodeError> {
    let (cargo, moves) = input
        .trim_end()
        .split_once("\n\n")
        .ok_or(AdventOfCodeError::InvalidInputError)?;

    let moves = get_moves(moves)?;
    let mut cargo = get_cargo(cargo)?;

    for move_ in moves {
        let from_stack = &mut cargo[move_.from - 1];

        let mut crates = from_stack.split_off(from_stack.len() - move_.count);

        let to_stack = &mut cargo[move_.to - 1];
        to_stack.append(&mut crates);
    }

    let top = cargo
        .iter()
        .enumerate()
        .map(|(i, stack)| {
            stack
                .back()
                .ok_or(AdventOfCodeError::EmptyStackError(i + 1))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect();

    Ok(top)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok("CMZ".to_owned()), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok("MCD".to_owned()), get_part_two(INPUT));
    }
}
