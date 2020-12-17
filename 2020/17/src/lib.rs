use std::collections::HashMap;
use std::str::FromStr;

type Coordinate = (isize, isize, isize, isize);

trait Neighbours<T> {
    fn get_neighbours(&self, dimensions: usize) -> Vec<T>;
}

impl Neighbours<Coordinate> for Coordinate {
    fn get_neighbours(&self, dimensions: usize) -> Vec<Coordinate> {
        let x = self.0;
        let y = self.1;
        let z = self.2;
        let w = self.3;

        (-1..=1)
            .flat_map(|dx| {
                (-1..=1)
                    .flat_map(|dy| {
                        (-1..=1)
                            .flat_map(|dz| {
                                let range = match dimensions {
                                    3 => (0..=0),
                                    4 => (-1..=1),
                                    _ => panic!("Invalid dimension: {}", dimensions),
                                };
                                range
                                    .map(|dw| (x + dx, y + dy, z + dz, w + dw))
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|neighbour| *neighbour != (x, y, z, w))
            .collect()
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
                    .map(|(x, c)| ((x as isize, y as isize, 0, 0), Cube::from(c)))
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

    pub fn cycle(&self, dimensions: usize) -> Grid {
        let ((min_x, min_y, min_z, min_w), (max_x, max_y, max_z, max_w)) = self.get_min_max();

        Grid {
            coordinates: ((min_x - 1)..=(max_x + 1))
                .flat_map(|x| {
                    ((min_y - 1)..=(max_y + 1))
                        .flat_map(move |y| {
                            ((min_z - 1)..=(max_z + 1))
                                .flat_map(move |z| {
                                    let range = match dimensions {
                                        3 => (0..=0),
                                        4 => ((min_w - 1)..=(max_w + 1)),
                                        _ => panic!("Invalid dimension: {}", dimensions),
                                    };
                                    range
                                        .map(move |w| {
                                            let cube = match self.coordinates.get(&(x, y, z, w)) {
                                                Some(cube) => cube.clone(),
                                                None => Cube::from('.'),
                                            };

                                            let active_neighbours = (x, y, z, w)
                                                .get_neighbours(dimensions)
                                                .iter()
                                                .filter(|neighbour| {
                                                    matches!(self.coordinates.get(neighbour), Some(cube) if cube.active)
                                                })
                                                .count();

                                            let active = match cube {
                                                cube if cube.active
                                                    && (2..=3).contains(&active_neighbours) =>
                                                {
                                                    true
                                                }
                                                cube if !cube.active && active_neighbours == 3 => {
                                                    true
                                                }
                                                _ => false,
                                            };

                                            ((x, y, z, w), Cube { active })
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<HashMap<_, _>>(),
        }
    }
}

pub fn get_grid_after_cycles(grid: Grid, dimensions: usize, cycles: usize) -> Grid {
    let mut grid = grid;
    for _ in 0..cycles {
        grid = grid.cycle(dimensions);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r##".#.
..#
###"##;

    #[test]
    fn test_part_one() {
        let grid = get_grid_after_cycles(Grid::from_str(&INPUT).unwrap(), 3, 6);

        assert_eq!(112, grid.get_active_cube_count(),);
    }

    #[test]
    fn test_part_two() {
        let grid = get_grid_after_cycles(Grid::from_str(&INPUT).unwrap(), 4, 6);

        assert_eq!(848, grid.get_active_cube_count(),);
    }
}
