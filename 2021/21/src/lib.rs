use std::collections::HashMap;

type Wins = (usize, usize);

struct Die {
    current: usize,
    rolls: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Player {
    position: usize,
    points: usize,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Game {
    player_1: Player,
    player_2: Player,
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

impl Player {
    fn get_move(&mut self, roll: usize) -> usize {
        self.position = (self.position + roll) % 10;
        if self.position == 0 {
            self.position = 10;
        }

        self.position
    }
}

// list(itertools.product(range(1, 4), repeat=3))
#[allow(dead_code)]
static ROLLS: &[[usize; 3]; 27] = &[
    [1, 1, 1],
    [1, 1, 2],
    [1, 1, 3],
    [1, 2, 1],
    [1, 2, 2],
    [1, 2, 3],
    [1, 3, 1],
    [1, 3, 2],
    [1, 3, 3],
    [2, 1, 1],
    [2, 1, 2],
    [2, 1, 3],
    [2, 2, 1],
    [2, 2, 2],
    [2, 2, 3],
    [2, 3, 1],
    [2, 3, 2],
    [2, 3, 3],
    [3, 1, 1],
    [3, 1, 2],
    [3, 1, 3],
    [3, 2, 1],
    [3, 2, 2],
    [3, 2, 3],
    [3, 3, 1],
    [3, 3, 2],
    [3, 3, 3],
];

// list(collections.Counter(sum(roll) for roll in itertools.product(range(1, 4), repeat=3)).items())
static ROLL_FREQUENCIES: &[(usize, usize)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

impl Game {
    fn get_wins(&mut self, cache: &mut HashMap<Game, Wins>) -> Wins {
        if self.player_1.points >= 21 {
            return (1, 0);
        }
        if self.player_2.points >= 21 {
            return (0, 1);
        }

        if let Some(&result) = cache.get(self) {
            return result;
        }

        let (mut player_1_win_count, mut player_2_win_count) = (0, 0);

        for &(roll, frequency) in ROLL_FREQUENCIES {
            let mut player_1 = self.player_1.clone();

            player_1.points += player_1.get_move(roll);

            let mut game = Game {
                player_1: self.player_2.clone(),
                player_2: player_1,
            };

            let (player_2_wins, player_1_wins) = game.get_wins(cache);

            player_1_win_count += frequency * player_1_wins;
            player_2_win_count += frequency * player_2_wins;
        }

        cache.insert(*self, (player_1_win_count, player_2_win_count));

        (player_1_win_count, player_2_win_count)
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
    let (player_1, player_2) = input.trim().split_once('\n').unwrap();

    let mut game = Game {
        player_1: Player {
            position: player_1.chars().last().unwrap().to_digit(10).unwrap() as usize,
            points: 0,
        },
        player_2: Player {
            position: player_2.chars().last().unwrap().to_digit(10).unwrap() as usize,
            points: 0,
        },
    };
    let (player_1_wins, player_2_wins) = game.get_wins(&mut HashMap::new());

    [player_1_wins, player_2_wins].into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

    #[test]
    fn test_part_one() {
        assert_eq!(739_785, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(444_356_092_776_315, get_part_two(INPUT));
    }
}
