use std::fs;

use ::day21::*;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let rules = Rules::from_str(&input).unwrap();
    let mut image = Image::new();
    for _ in 0..5 {
        image = image.get_iteration(&rules);
    }

    println!(
        "How many pixels stay on after 5 iterations? {}",
        image.get_on_count(),
    );
}
