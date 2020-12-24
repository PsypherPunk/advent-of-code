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
}
