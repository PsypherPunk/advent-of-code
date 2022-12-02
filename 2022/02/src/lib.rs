enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (opponent, you) = line.split_once(' ').unwrap();
            let opponent = match opponent {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let you = match you {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => unreachable!(),
            };

            match opponent {
                Shape::Rock => match you {
                    Shape::Rock => 3 + 1,
                    Shape::Paper => 6 + 2,
                    Shape::Scissors => 0 + 3,
                },
                Shape::Paper => match you {
                    Shape::Rock => 0 + 1,
                    Shape::Paper => 3 + 2,
                    Shape::Scissors => 6 + 3,
                },
                Shape::Scissors => match you {
                    Shape::Rock => 6 + 1,
                    Shape::Paper => 0 + 2,
                    Shape::Scissors => 3 + 3,
                },
            }
        })
        .sum()
}

pub fn get_part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (opponent, you) = line.split_once(' ').unwrap();
            let opponent = match opponent {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => unreachable!(),
            };
            let you = match you {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!(),
            };

            match opponent {
                Shape::Rock => match you {
                    Outcome::Win => 6 + 2,
                    Outcome::Lose => 0 + 3,
                    Outcome::Draw => 3 + 1,
                },
                Shape::Paper => match you {
                    Outcome::Win => 6 + 3,
                    Outcome::Lose => 0 + 1,
                    Outcome::Draw => 3 + 2,
                },
                Shape::Scissors => match you {
                    Outcome::Win => 6 + 1,
                    Outcome::Lose => 0 + 2,
                    Outcome::Draw => 3 + 3,
                },
            }
        })
        .sum()
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
        assert_eq!(15, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(12, get_part_two(INPUT));
    }
}
