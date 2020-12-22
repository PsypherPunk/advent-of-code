use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Game {
    player_1: VecDeque<usize>,
    player_2: VecDeque<usize>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (player_1, player_2) = match s.trim().splitn(2, "\n\n").collect::<Vec<_>>()[..] {
            [a, b] => (a, b),
            _ => panic!("Invalid input."),
        };

        Ok(Game {
            player_1: Game::get_deck(player_1),
            player_2: Game::get_deck(player_2),
        })
    }
}

impl Game {
    fn get_deck(deck: &str) -> VecDeque<usize> {
        deck.trim()
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()
    }

    fn get_highest_score(&self) -> usize {
        let player_1_score = self
            .player_1
            .iter()
            .rev()
            .enumerate()
            .map(|(position, card)| (position + 1) * card)
            .sum::<usize>();

        let player_2_score = self
            .player_2
            .iter()
            .rev()
            .enumerate()
            .map(|(position, card)| (position + 1) * card)
            .sum::<usize>();

        player_1_score.max(player_2_score)
    }

    pub fn get_combat_winning_score(&mut self) -> usize {
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            let player_1_card = self.player_1.pop_front().unwrap();
            let player_2_card = self.player_2.pop_front().unwrap();
            match player_1_card.cmp(&player_2_card) {
                Ordering::Greater => {
                    self.player_1.push_back(player_1_card);
                    self.player_1.push_back(player_2_card);
                }
                Ordering::Less => {
                    self.player_2.push_back(player_2_card);
                    self.player_2.push_back(player_1_card);
                }
                _ => panic!("Invalid round: {}, {}", player_1_card, player_2_card),
            }
        }

        self.get_highest_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    #[test]
    fn test_part_one() {
        let mut game = Game::from_str(&INPUT).unwrap();

        assert_eq!(306, game.get_combat_winning_score());
    }
}
