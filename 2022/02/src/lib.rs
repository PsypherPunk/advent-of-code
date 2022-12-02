use std::cmp::Ordering;
use std::str::FromStr;

struct Round {
    you: Shape,
    opponent: Shape,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, you) = s
            .split_once(' ')
            .ok_or_else(|| format!("invalid line: {}", s))?;

        Ok(Self {
            you: Shape::from_str(you)?,
            opponent: Shape::from_str(opponent)?,
        })
    }
}

impl Round {
    fn get_score(self) -> usize {
        let outcome = match &self.you.partial_cmp(&self.opponent) {
            Some(Ordering::Equal) => Outcome::Draw as usize,
            Some(Ordering::Less) => Outcome::Lose as usize,
            Some(Ordering::Greater) => Outcome::Win as usize,
            None => unreachable!(),
        };

        outcome + self.you as usize
    }
}

#[derive(PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(format!("invalid shape: {}", s)),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Some(Ordering::Equal),
                Shape::Paper => Some(Ordering::Less),
                Shape::Scissors => Some(Ordering::Greater),
            },
            Shape::Paper => match other {
                Shape::Rock => Some(Ordering::Greater),
                Shape::Paper => Some(Ordering::Equal),
                Shape::Scissors => Some(Ordering::Less),
            },
            Shape::Scissors => match other {
                Shape::Rock => Some(Ordering::Less),
                Shape::Paper => Some(Ordering::Greater),
                Shape::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl Shape {
    #[allow(clippy::wrong_self_convention)]
    fn from_outcome(&self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Lose => match self {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => match self {
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            },
        }
    }
}

enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("invalid outcome: {}", s)),
        }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let score = input
        .trim()
        .lines()
        .map(|line| {
            let round = Round::from_str(line)?;

            Ok::<usize, String>(round.get_score())
        })
        .collect::<Result<Vec<usize>, _>>()?
        .iter()
        .sum();

    Ok(score)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let score = input
        .trim()
        .lines()
        .map(|line| {
            let (opponent, outcome) = line
                .split_once(' ')
                .ok_or_else(|| format!("invalid line: {}", line))?;

            let (opponent, outcome) = (Shape::from_str(opponent)?, Outcome::from_str(outcome)?);

            let round = Round {
                you: opponent.from_outcome(&outcome),
                opponent,
            };

            Ok::<usize, String>(round.get_score())
        })
        .collect::<Result<Vec<usize>, _>>()?
        .iter()
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(15), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(12), get_part_two(INPUT));
    }
}
