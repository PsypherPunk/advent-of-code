use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
struct Move {
    count: usize,
    from: char,
    to: char,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

        Ok(Move {
            count: parts[1].parse().unwrap(),
            from: parts[3].chars().next().unwrap(),
            to: parts[5].chars().next().unwrap(),
        })
    }
}

fn get_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(Move::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn get_cargo(input: &str) -> HashMap<char, VecDeque<char>> {
    let stacks = input.lines().last().unwrap();

    let cargo = input.lines().rev().skip(1).collect::<Vec<_>>();

    stacks
        .chars()
        .enumerate()
        .filter_map(|(position, c)| match c {
            ' ' => None,
            _ => Some((position, c)),
        })
        .map(|(position, c)| {
            let stacked = cargo
                .iter()
                .map(move |line| line.chars().nth(position).unwrap())
                .filter(|c| *c != ' ')
                .collect::<VecDeque<_>>();

            (c, stacked)
        })
        .collect()
}

pub fn get_part_one(input: &str) -> String {
    let (cargo, moves) = input.trim_end().split_once("\n\n").unwrap();

    let moves = get_moves(moves);
    let mut cargo = get_cargo(cargo);

    for move_ in moves {
        for _ in 0..move_.count {
            let from_stack = cargo.get_mut(&move_.from).unwrap();
            let crate_ = from_stack.pop_back().unwrap();

            let to_stack = cargo.get_mut(&move_.to).unwrap();
            to_stack.push_back(crate_);
        }
    }

    let mut stacks = cargo.keys().collect::<Vec<_>>();
    stacks.sort();

    stacks
        .iter()
        .map(|stack| {
            let stacked = cargo.get(stack).unwrap();
            stacked.back().unwrap()
        })
        .collect::<String>()
}

pub fn get_part_two(input: &str) -> String {
    let (cargo, moves) = input.trim_end().split_once("\n\n").unwrap();

    let moves = get_moves(moves);
    let mut cargo = get_cargo(cargo);

    for move_ in moves {
        let mut crate_mover = Vec::new();
        let from_stack = cargo.get_mut(&move_.from).unwrap();

        for _ in 0..move_.count {
            let crate_ = from_stack.pop_back().unwrap();

            crate_mover.push(crate_);
        }
        let to_stack = cargo.get_mut(&move_.to).unwrap();
        to_stack.extend(crate_mover.iter().rev());
    }

    let mut stacks = cargo.keys().collect::<Vec<_>>();
    stacks.sort();

    stacks
        .iter()
        .map(|stack| {
            let stacked = cargo.get(stack).unwrap();
            stacked.back().unwrap()
        })
        .collect::<String>()
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
        assert_eq!("CMZ", get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!("MCD", get_part_two(INPUT));
    }
}
