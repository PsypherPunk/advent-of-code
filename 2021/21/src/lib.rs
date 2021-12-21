struct Die {
    current: usize,
    rolls: usize,
}

impl Die {
    fn get_roll(&mut self) -> usize {
        self.current = (self.current + 1) % 100;
        if self.current == 0 {
            self.current = 100;
        }
        self.rolls += 1;

        self.current
    }
}

struct Player {
    position: usize,
    points: usize,
}

impl Player {
    fn get_move(&mut self, roll: usize) -> usize {
        self.position = (self.position + roll) % 10;
        if self.position == 0 {
            self.position = 10;
        }

        self.position
    }
}

pub fn get_part_one(input: &str) -> usize {
    let (player_1, player_2) = input.trim().split_once('\n').unwrap();
    let mut player_1 = Player {
        position: player_1.chars().last().unwrap().to_digit(10).unwrap() as usize,
        points: 0,
    };
    let mut player_2 = Player {
        position: player_2.chars().last().unwrap().to_digit(10).unwrap() as usize,
        points: 0,
    };

    let mut die = Die {
        current: 0,
        rolls: 0,
    };

    loop {
        player_1.get_move(die.get_roll());
        player_1.get_move(die.get_roll());
        player_1.points += player_1.get_move(die.get_roll());
        if player_1.points >= 1_000 {
            break;
        }

        player_2.get_move(die.get_roll());
        player_2.get_move(die.get_roll());
        player_2.points += player_2.get_move(die.get_roll());
        if player_2.points >= 1_000 {
            break;
        }
    }

    let loser = [player_1, player_2]
        .into_iter()
        .min_by_key(|player| player.points)
        .unwrap();

    loser.points * die.rolls
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

    #[test]
    fn test_part_one() {
        assert_eq!(739785, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
