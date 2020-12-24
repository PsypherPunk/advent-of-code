use std::fs;
use std::str::FromStr;

use ::day24::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let lobby = Lobby::from_str(&input).unwrap();

    println!(
        "…how many tiles are left with the black side up? {}",
        lobby.get_black_tile_count(),
    );

    let mut lobby = Lobby::from_str(&input).unwrap();
    for _ in 1..=100 {
        lobby = lobby.get_next_day();
    }
    println!(
        "How many tiles will be black after 100 days? {}",
        lobby.get_black_tile_count(),
    );
}
