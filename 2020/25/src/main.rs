use std::fs;

use ::day25::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let (card_public_key, door_public_key) = get_public_keys(&input);
    let card_loop_size = get_loop_size(card_public_key);

    println!(
        "What encryption key is the handshake trying to establish? {}",
        get_encryption_key(door_public_key, card_loop_size),
    );
}
