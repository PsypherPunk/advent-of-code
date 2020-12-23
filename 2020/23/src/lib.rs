use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct CrabCups {
    current_cup: usize,
    cups: VecDeque<usize>,
}

impl FromStr for CrabCups {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups: VecDeque<usize> = s
            .trim()
            .chars()
            .map(|ch| char::to_digit(ch, 10).unwrap() as usize)
            .collect();

        Ok(Self {
            current_cup: *cups.get(0).unwrap(),
            cups,
        })
    }
}

impl CrabCups {
    fn get_cup_position(&self, cup: &usize) -> usize {
        self.cups.iter().position(|c| c == cup).unwrap()
    }

    pub fn do_move(&mut self) {
        let mut three_cups = Vec::new();
        for _ in 1..=3 {
            let removed = self
                .cups
                .remove((self.get_cup_position(&self.current_cup) + 1) % self.cups.len())
                .unwrap();
            three_cups.push(removed);
        }

        let mut destination_cup = match self.current_cup {
            1 => *self.cups.iter().max().unwrap(),
            _ => self.current_cup - 1,
        };
        while three_cups.contains(&destination_cup) {
            destination_cup = match destination_cup {
                1 => *self.cups.iter().max().unwrap(),
                _ => destination_cup - 1,
            };
        }

        let destination_cup_position = self
            .cups
            .iter()
            .position(|c| *c == destination_cup)
            .unwrap();
        three_cups
            .iter()
            .enumerate()
            .for_each(|(i, cup)| self.cups.insert(destination_cup_position + 1 + i, *cup));

        let current_cup_position = self.get_cup_position(&self.current_cup);
        self.current_cup = match (current_cup_position + 1).cmp(&self.cups.len()) {
            Ordering::Greater | Ordering::Equal => *self.cups.get(0).unwrap(),
            Ordering::Less => *self.cups.get(current_cup_position + 1).unwrap(),
        };
    }

    pub fn get_labels(&self) -> String {
        let one_position = self.cups.iter().position(|c| *c == 1).unwrap();

        let mut after = self.cups.iter().skip(one_position + 1).collect::<Vec<_>>();
        let before = self.cups.iter().take(one_position).collect::<Vec<_>>();
        after.extend(before.iter());

        after
            .iter()
            .map(|cup| cup.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"389125467"#;

    #[test]
    fn test_part_one() {
        let mut crab_cups = CrabCups::from_str(&INPUT).unwrap();

        for _ in 0..10 {
            crab_cups.do_move();
        }

        assert_eq!("92658374", crab_cups.get_labels());
    }
}
