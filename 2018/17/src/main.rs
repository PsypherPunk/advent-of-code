use std::fs;
use std::str::FromStr;

use ::day17::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut grid = Grid::from_str(&input).unwrap();

    dbg!(grid.get_square_metres_water());
}
