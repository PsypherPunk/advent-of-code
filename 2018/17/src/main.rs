use std::fs;
use std::str::FromStr;

use ::day17::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut grid = Grid::from_str(&input).unwrap();

    let (total, rest) = grid.get_square_metres_water();

    println!("How many tiles can the water reach…? {}", total,);

    println!("How many water tiles are left…? {}", rest,);
}
