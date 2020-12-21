use std::fs;
use std::str::FromStr;

use ::day20::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut image = Image::from_str(&input).unwrap();
    println!(
        "What do you get if you multiply together the IDs of the four corner tiles? {}",
        image.get_corners().iter().product::<usize>(),
    );

    let mut image = Image::from_str(&input).unwrap();
    let tile = image.get_final_image();
    println!(
        "How many # are not part of a sea monster? {}",
        tile.find_sea_monsters(),
    );
}
