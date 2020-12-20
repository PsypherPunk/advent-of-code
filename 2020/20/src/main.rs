use std::fs;
use std::str::FromStr;

use ::day20::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let image = Image::from_str(&input).unwrap();

    println!(
        "What do you get if you multiply together the IDs of the four corner tiles? {}",
        image.get_borders().iter().product::<usize>(),
    );
}
