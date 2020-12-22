use std::fs;
use std::str::FromStr;

use ::day22::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut game = Game::from_str(&input).unwrap();

    println!(
        "What is the winning player's score? {}",
        game.get_combat_winning_score(),
    );

    let mut game = Game::from_str(&input).unwrap();

    println!(
        "What is the winning player's score? {}",
        game.get_recursive_combat_winning_score(),
    );
}
