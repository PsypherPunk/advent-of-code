use std::fs;

use ::day24::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut reindeer = reindeer::reindeer(&input).unwrap();

    println!(
        "…how many units would the winning army have? {}",
        reindeer.fight().unwrap(),
    );

    println!(
        "How many units does the immune system have left…? {}",
        get_immune_system_boost(&input),
    );
}
