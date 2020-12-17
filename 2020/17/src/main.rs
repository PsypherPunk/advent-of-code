use std::fs;
use std::str::FromStr;

use ::day17::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let grid = get_grid_after_cycles(Grid::from_str(&input).unwrap(), 3, 6);
    println!(
        "How many cubes are left in the active state after the sixth cycle? {}",
        grid.get_active_cube_count(),
    );

    let grid = get_grid_after_cycles(Grid::from_str(&input).unwrap(), 4, 6);
    println!(
        "How many cubes are left in the active state after the sixth cycle? {}",
        grid.get_active_cube_count(),
    );
}
