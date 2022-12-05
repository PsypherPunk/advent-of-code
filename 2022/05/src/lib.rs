#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::{HashMap, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidInputError,
    ParseIntError(ParseIntError),
    UnexpectedStackError(char),
    EmptyStackError(char),
}

impl From<ParseIntError> for AdventOfCodeError {
    fn from(error: ParseIntError) -> Self {
        AdventOfCodeError::ParseIntError(error)
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: char,
    to: char,
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
                .ok_or(AdventOfCodeError::InvalidInputError)?,
            to: parts[5]
                .chars()
                .next()
                .ok_or(AdventOfCodeError::InvalidInputError)?,
        })
    }
}

fn get_moves(input: &str) -> Result<Vec<Move>, AdventOfCodeError> {
    input.lines().map(Move::from_str).collect()
}

fn get_cargo(input: &str) -> Result<HashMap<char, VecDeque<char>>, AdventOfCodeError> {
    let stacks = input
        .lines()
        .last()
        .ok_or(AdventOfCodeError::InvalidInputError)?;

    let cargo = input.lines().rev().skip(1).collect::<Vec<_>>();

    let cargo = stacks
        .chars()
        .enumerate()
        .filter_map(|(position, c)| match c {
            ' ' => None,
            _ => Some((position, c)),
        })
        .map(|(position, c)| {
            let stacked = cargo
                .iter()
                .map(move |line| match line.chars().nth(position) {
                    None => Err(AdventOfCodeError::InvalidInputError),
                    Some(c) => Ok(c),
                })
                .filter(|c| *c != Ok(' '))
                .collect::<Result<VecDeque<_>, _>>();

            match stacked {
                Ok(s) => Ok((c, s)),
                _ => Err(AdventOfCodeError::InvalidInputError),
            }
        })
        .collect::<Result<HashMap<char, _>, _>>()?;

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
        for _ in 0..move_.count {
            let from_stack = cargo
                .get_mut(&move_.from)
                .ok_or(AdventOfCodeError::UnexpectedStackError(move_.from))?;
            let crate_ = from_stack
                .pop_back()
                .ok_or(AdventOfCodeError::EmptyStackError(move_.from))?;

            let to_stack = cargo
                .get_mut(&move_.to)
                .ok_or(AdventOfCodeError::UnexpectedStackError(move_.to))?;
            to_stack.push_back(crate_);
        }
    }

    let mut stacks = cargo.keys().collect::<Vec<_>>();
    stacks.sort();

    let top = stacks
        .iter()
        .map(|stack| match cargo.get(stack) {
            Some(stacked) => stacked.back().ok_or(AdventOfCodeError::InvalidInputError),
            None => Err(AdventOfCodeError::InvalidInputError),
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
        let mut crate_mover = Vec::new();
        let from_stack = cargo
            .get_mut(&move_.from)
            .ok_or(AdventOfCodeError::UnexpectedStackError(move_.from))?;

        for _ in 0..move_.count {
            let crate_ = from_stack
                .pop_back()
                .ok_or(AdventOfCodeError::EmptyStackError(move_.from))?;

            crate_mover.push(crate_);
        }
        let to_stack = cargo
            .get_mut(&move_.to)
            .ok_or(AdventOfCodeError::UnexpectedStackError(move_.to))?;
        to_stack.extend(crate_mover.iter().rev());
    }

    let mut stacks = cargo.keys().collect::<Vec<_>>();
    stacks.sort();

    let top = stacks
        .iter()
        .map(|stack| match cargo.get(stack) {
            Some(stacked) => stacked.back().ok_or(AdventOfCodeError::InvalidInputError),
            None => Err(AdventOfCodeError::InvalidInputError),
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
