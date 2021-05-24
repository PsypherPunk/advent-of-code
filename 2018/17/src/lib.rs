use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum Tile {
    Sand,
    Clay,
    WaterFlow,
    WaterRest,
}

pub struct Grid {
    tiles: HashMap<(usize, usize), Tile>,
    min: (usize, usize),
}

#[derive(Debug)]
pub struct Clay {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

enum Direction {
    Left,
    Right,
    Both,
}

peg::parser! {
    grammar scan() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule x_range() -> Clay
            = "x=" x:integer() ", y=" y_s:integer() ".." y_e:integer()
                { Clay { x: x..=x, y: y_s..=y_e } }

        rule y_range() -> Clay
            = "y=" y:integer() ", x=" x_s:integer() ".." x_e:integer()
                { Clay { x: x_s..=x_e, y: y..=y } }

        pub rule clay() -> Vec<Clay>
            = c:(x_range() / y_range())
              ++ _
              _
                { c }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clay = scan::clay(&s).unwrap();

        let min_x = clay.iter().map(|c| *c.x.start()).min().unwrap();
        let min_y = clay.iter().map(|c| *c.y.start()).min().unwrap();
        let max_x = clay.iter().map(|c| *c.x.end()).max().unwrap();
        let max_y = clay.iter().map(|c| *c.y.end()).max().unwrap();

        let tiles = (0..=(max_y - min_y))
            .flat_map(|y| (0..=(max_x - min_x)).map(move |x| ((x, y), Tile::Sand)))
            .collect::<HashMap<_, _>>();

        let mut grid = Grid {
            tiles,
            min: (min_x, min_y),
        };

        clay.into_iter()
            .flat_map(|c: Clay| {
                c.y.clone()
                    .flat_map(move |y| c.x.clone().map(move |x| (x, y)))
            })
            .for_each(|(x, y)| {
                grid.tiles.insert((x - min_x + 1, y - min_y), Tile::Clay);
            });

        Ok(grid)
    }
}

impl Grid {
    pub fn get_square_metres_water(&mut self) -> (usize, usize) {
        let left = 500 - self.min.0 + 1;
        self.get_tile(left, 0, &Direction::Both);

        self.tiles
            .values()
            .map(|square_meter| {
                (
                    matches!(square_meter, Tile::WaterFlow | Tile::WaterRest) as usize,
                    matches!(square_meter, Tile::WaterRest) as usize,
                )
            })
            .fold((0, 0), |(total_water, total_rest), (water, rest)| {
                (total_water + water, total_rest + rest)
            })
    }

    fn get_tile(&mut self, x: usize, y: usize, direction: &Direction) -> Option<usize> {
        if y == self.tiles.len() {
            return None;
        }

        match self.tiles.get(&(x, y))? {
            Tile::Clay | Tile::WaterRest => Some(x),
            Tile::WaterFlow => None,
            Tile::Sand => {
                self.tiles.insert((x, y), Tile::WaterFlow);
                self.get_tile(x, y + 1, &Direction::Both)?;
                match direction {
                    Direction::Both => {
                        match (
                            self.get_tile(x - 1, y, &Direction::Left),
                            self.get_tile(x + 1, y, &Direction::Right),
                        ) {
                            (Some(left), Some(right)) => {
                                let water = self
                                    .tiles
                                    .keys()
                                    .filter(|(row, col)| {
                                        *col == y && *row >= left + 1 && *row <= right - 1
                                    })
                                    .map(|tile| (*tile, Tile::WaterRest))
                                    .collect::<HashMap<_, _>>();
                                self.tiles.extend(water);

                                Some(x)
                            }
                            _ => None,
                        }
                    }
                    Direction::Left => self.get_tile(x - 1, y, &Direction::Left),
                    Direction::Right => self.get_tile(x + 1, y, &Direction::Right),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;

    #[test]
    fn test_part_one() {
        let mut grid = Grid::from_str(&INPUT).unwrap();

        assert_eq!(57, grid.get_square_metres_water().0);
    }

    #[test]
    fn test_part_two() {
        let mut grid = Grid::from_str(&INPUT).unwrap();

        assert_eq!(29, grid.get_square_metres_water().1);
    }
}
