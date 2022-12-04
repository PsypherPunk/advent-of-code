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
        let outcome = match self.you {
            Shape::Rock => match self.opponent {
                Shape::Rock => Outcome::Draw as usize,
                Shape::Paper => Outcome::Lose as usize,
                Shape::Scissors => Outcome::Win as usize,
            },
            Shape::Paper => match self.opponent {
                Shape::Rock => Outcome::Win as usize,
                Shape::Paper => Outcome::Draw as usize,
                Shape::Scissors => Outcome::Lose as usize,
            },
            Shape::Scissors => match self.opponent {
                Shape::Rock => Outcome::Lose as usize,
                Shape::Paper => Outcome::Win as usize,
                Shape::Scissors => Outcome::Draw as usize,
            },
        };

        outcome + self.you as usize
    }
}

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
        .sum::<Result<usize, _>>()?;

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
        .sum::<Result<usize, String>>()?;

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
