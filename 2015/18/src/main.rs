use std::collections::HashMap;
use std::fs;

type Point = (isize, isize);

fn get_grid(input: &str) -> HashMap<Point, bool> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => ((x as isize, y as isize), false),
                    '#' => ((x as isize, y as isize), true),
                    _ => panic!("Invalid character: {}", c),
                })
                .collect::<Vec<(Point, bool)>>()
        })
        .collect()
}

fn get_neighbours((x, y): &Point) -> Vec<Point> {
    vec![
        (x - 1, y - 1),
        (*x, y - 1),
        (x + 1, y - 1),
        (x - 1, *y),
        (x + 1, *y),
        (x - 1, y + 1),
        (*x, y + 1),
        (x + 1, y + 1),
    ]
}

fn get_lit_neighbour_count(light: &Point, grid: &HashMap<Point, bool>) -> usize {
    get_neighbours(light)
        .iter()
        .map(|neighbour| match grid.get(neighbour) {
            Some(&state) => state,
            None => false,
        })
        .filter(|&state| state)
        .count()
}

fn get_lit_count(grid: &HashMap<Point, bool>) -> usize {
    grid.values().filter(|&state| *state).count()
}

fn animate_grid(grid: &HashMap<Point, bool>) -> HashMap<Point, bool> {
    let (width, height) = grid.keys().max().unwrap();

    (0..=*height)
        .flat_map(|y| {
            (0..=*width)
                .map(move |x| {
                    let lit_neighbour_count = get_lit_neighbour_count(&(x, y), &grid);
                    match grid.get(&(x, y)).unwrap() {
                        true => match lit_neighbour_count {
                            2 | 3 => ((x, y), true),
                            _ => ((x, y), false),
                        },
                        false => match lit_neighbour_count {
                            3 => ((x, y), true),
                            _ => ((x, y), false),
                        },
                    }
                })
                .collect::<Vec<(Point, bool)>>()
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut grid = get_grid(&input);
    for _ in 0..100 {
        grid = animate_grid(&grid);
    }

    println!(
        "…how many lights are on after 100 steps? {}",
        get_lit_count(&grid),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        let mut grid = get_grid(&input);

        for _ in 0..4 {
            grid = animate_grid(&grid);
        }

        assert_eq!(4, get_lit_count(&grid));
    }
}
