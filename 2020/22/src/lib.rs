use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

type Player = VecDeque<usize>;

pub struct Game {
    player_1: Player,
    player_2: Player,
    hands: HashSet<usize>,
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
            hands: HashSet::new(),
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

    fn get_player_score(&self, player: &Player) -> usize {
        player
            .iter()
            .rev()
            .enumerate()
            .map(|(position, card)| (position + 1) * card)
            .sum()
    }

    fn get_checksum(&self) -> usize {
        self.get_player_score(&self.player_1) * self.get_player_score(&self.player_2)
    }

    fn player_one_wins(&self) -> bool {
        match self.player_1.is_empty() {
            true => false,
            false => match self.player_2.is_empty() {
                true => true,
                false => true,
            },
        }
    }

    fn play_single_round(&mut self, player_1_card: usize, player_2_card: usize) {
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

    pub fn get_combat_winning_score(&mut self) -> usize {
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            let checksum = self.get_checksum();
            if self.hands.contains(&checksum) {
                break;
            }

            let player_1_card = self.player_1.pop_front().unwrap();
            let player_2_card = self.player_2.pop_front().unwrap();
            self.play_single_round(player_1_card, player_2_card);
        }

        match self.player_one_wins() {
            true => self.get_player_score(&self.player_1),
            false => self.get_player_score(&self.player_2),
        }
    }

    fn get_recursive_combat_winner(&mut self) -> bool {
        while !self.player_1.is_empty() && !self.player_2.is_empty() {
            let checksum = self.get_checksum();
            if self.hands.contains(&checksum) {
                break;
            }
            self.hands.insert(checksum);

            let player_1_card = self.player_1.pop_front().unwrap();
            let player_2_card = self.player_2.pop_front().unwrap();

            if player_1_card <= self.player_1.len() && player_2_card <= self.player_2.len() {
                let mut game = Game {
                    player_1: self.player_1.clone(),
                    player_2: self.player_2.clone(),
                    hands: HashSet::new(),
                };
                game.player_1.resize(player_1_card, 0);
                game.player_2.resize(player_2_card, 0);
                match game.get_recursive_combat_winner() {
                    true => {
                        self.player_1.push_back(player_1_card);
                        self.player_1.push_back(player_2_card);
                    }
                    false => {
                        self.player_2.push_back(player_2_card);
                        self.player_2.push_back(player_1_card);
                    }
                }
            } else {
                self.play_single_round(player_1_card, player_2_card);
            }
        }

        self.player_one_wins()
    }

    pub fn get_recursive_combat_winning_score(&mut self) -> usize {
        match self.get_recursive_combat_winner() {
            true => self.get_player_score(&self.player_1),
            false => self.get_player_score(&self.player_2),
        }
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

    #[test]
    fn test_part_two() {
        let mut game = Game::from_str(&INPUT).unwrap();

        assert_eq!(291, game.get_recursive_combat_winning_score());
    }
}
