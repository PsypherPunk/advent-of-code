use std::collections::HashMap;
use std::str::FromStr;

type Coordinate = (isize, isize, isize);

trait Neighbours<T> {
    fn get_neighbours(&self) -> Vec<T>;
}

impl Neighbours<Coordinate> for Coordinate {
    fn get_neighbours(&self) -> Vec<Coordinate> {
        let x = self.0;
        let y = self.1;
        let z = self.2;
        vec![
            (x - 1, y - 1, z),
            (x, y - 1, z),
            (x + 1, y - 1, z),
            (x + 1, y, z),
            (x + 1, y + 1, z),
            (x, y + 1, z),
            (x - 1, y + 1, z),
            (x - 1, y, z),
            (x - 1, y - 1, z - 1),
            (x, y - 1, z - 1),
            (x + 1, y - 1, z - 1),
            (x + 1, y, z - 1),
            (x + 1, y + 1, z - 1),
            (x, y + 1, z - 1),
            (x - 1, y + 1, z - 1),
            (x - 1, y, z - 1),
            (x, y, z - 1),
            (x - 1, y - 1, z + 1),
            (x, y - 1, z + 1),
            (x + 1, y - 1, z + 1),
            (x + 1, y, z + 1),
            (x + 1, y + 1, z + 1),
            (x, y + 1, z + 1),
            (x - 1, y + 1, z + 1),
            (x - 1, y, z + 1),
            (x, y, z + 1),
        ]
    }
}

#[derive(Clone)]
struct Cube {
    active: bool,
}

#[derive(Clone)]
pub struct Grid {
    coordinates: HashMap<Coordinate, Cube>,
}

impl From<char> for Cube {
    fn from(c: char) -> Self {
        let active = match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid character for Cube: {}", c),
        };
        Self { active }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| ((x as isize, y as isize, 0), Cube::from(c)))
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(Self { coordinates })
    }
}

impl Grid {
    fn get_min_max(&self) -> (Coordinate, Coordinate) {
        (
            *self.coordinates.keys().min().unwrap(),
            *self.coordinates.keys().max().unwrap(),
        )
    }

    pub fn get_active_cube_count(&self) -> usize {
        self.coordinates.values().filter(|cube| cube.active).count()
    }

    pub fn cycle(&self) -> Grid {
        let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = self.get_min_max();

        Grid {
            coordinates: ((min_x - 1)..=(max_x + 1))
                .flat_map(|x| {
                    ((min_y - 1)..=(max_y + 1))
                        .flat_map(move |y| {
                            ((min_z - 1)..=(max_z + 1))
                                .map(move |z| {
                                    let cube = match self.coordinates.get(&(x, y, z)) {
                                        Some(cube) => cube.clone(),
                                        None => Cube::from('.'),
                                    };

                                    let active_neighbours = (x, y, z)
                                        .get_neighbours()
                                        .iter()
                                        .map(|neighbour| match self.coordinates.get(neighbour) {
                                            Some(cube) if cube.active => 1,
                                            _ => 0,
                                        })
                                        .sum::<usize>();

                                    let active = match cube {
                                        cube if cube.active
                                            && (2..=3).contains(&active_neighbours) =>
                                        {
                                            true
                                        }
                                        cube if !cube.active && active_neighbours == 3 => true,
                                        _ => false,
                                    };

                                    ((x, y, z), Cube { active })
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

pub fn get_grid_after_cycles(grid: Grid, cycles: usize) -> Grid {
    let mut grid = grid;
    for _ in 0..cycles {
        grid = grid.cycle();
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r##".#.
..#
###"##;

        let grid = get_grid_after_cycles(Grid::from_str(&input).unwrap(), 6);

        assert_eq!(112, grid.get_active_cube_count(),);
    }
}
