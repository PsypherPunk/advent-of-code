use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

type Point = (isize, isize);

const SEA_MONSTER: &'static str = r##"                  #
#    ##    ##    ###
 #  #  #  #  #  #"##;

#[derive(Clone, PartialEq)]
enum Pixel {
    White,
    Black,
}

#[derive(Clone)]
struct Border {
    top: u128,
    right: u128,
    bottom: u128,
    left: u128,
}

#[derive(Clone)]
pub struct Tile {
    pub id: usize,
    pixels: HashMap<Point, Pixel>,
    border: Border,
    adjacent: (Option<usize>, Option<usize>, Option<usize>, Option<usize>),
}

pub struct Image {
    tiles: Vec<Tile>,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let px = match self {
            Pixel::White => '⬜',
            Pixel::Black => '⬛',
        };

        write!(f, "{}", px)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.pixels.keys().max().unwrap();

        let display = (0..=*max_y)
            .map(|y| {
                (0..=*max_x)
                    .map(|x| match self.pixels.get(&(x, y)).unwrap() {
                        Pixel::White => '⬜',
                        Pixel::Black => '⬛',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let display = format!(
            "Tile ID: {}\n{}\n{: ^13}\n{: <4}     {: >4}\n{: ^13}\n",
            self.id,
            display,
            self.border.top,
            self.border.left,
            self.border.right,
            self.border.bottom,
        );

        write!(f, "{}", display)
    }
}

impl Border {
    fn from_pixels(pixels: &HashMap<Point, Pixel>) -> Self {
        let (max_x, max_y) = pixels.keys().max().unwrap();

        let top = (0..=*max_x)
            .enumerate()
            .map(|(index, x)| {
                let bit = match pixels.get(&(x, 0)).unwrap() {
                    Pixel::White => 0,
                    Pixel::Black => 1,
                };
                bit << index
            })
            .sum();
        let right = (0..=*max_y)
            .enumerate()
            .map(|(index, y)| {
                let bit = match pixels.get(&(*max_x, y)).unwrap() {
                    Pixel::White => 0,
                    Pixel::Black => 1,
                };
                bit << index
            })
            .sum();
        let bottom = (0..=*max_x)
            .rev()
            .enumerate()
            .map(|(index, x)| {
                let bit = match pixels.get(&(x, *max_y)).unwrap() {
                    Pixel::White => 0,
                    Pixel::Black => 1,
                };
                bit << index
            })
            .sum();
        let left = (0..=*max_y)
            .rev()
            .enumerate()
            .map(|(index, y)| {
                let bit = match pixels.get(&(0, y)).unwrap() {
                    Pixel::White => 0,
                    Pixel::Black => 1,
                };
                bit << index
            })
            .sum();

        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.trim().lines().next().unwrap()[5..=8].parse().unwrap();

        let pixels: HashMap<Point, Pixel> = s
            .trim()
            .lines()
            .skip(1)
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        (x as isize, y as isize),
                        match c {
                            '.' => Pixel::White,
                            '#' => Pixel::Black,
                            _ => panic!("Invalid character: {}", c),
                        },
                    )
                })
            })
            .collect();

        let border = Border::from_pixels(&pixels);

        Ok(Self {
            id,
            pixels,
            border,
            adjacent: (None, None, None, None),
        })
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .trim()
            .split("\n\n")
            .flat_map(|tile| {
                let mut permutations = Vec::new();
                let tile = Tile::from_str(tile).unwrap();
                permutations.push(tile.clone());
                permutations.push(tile.rotate_anti_clockwise());
                permutations.push(tile.rotate_anti_clockwise().rotate_anti_clockwise());
                permutations.push(
                    tile.rotate_anti_clockwise()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise(),
                );
                permutations.push(tile.flip_x());
                permutations.push(tile.flip_x().rotate_anti_clockwise());
                permutations.push(
                    tile.flip_x()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise(),
                );
                permutations.push(
                    tile.flip_x()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise(),
                );
                permutations.push(tile.flip_y());
                permutations.push(tile.flip_y().rotate_anti_clockwise());
                permutations.push(
                    tile.flip_y()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise(),
                );
                permutations.push(
                    tile.flip_y()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise()
                        .rotate_anti_clockwise(),
                );
                permutations
            })
            .collect();

        let mut image = Image { tiles };
        image.find_adjacent_tiles();

        Ok(image)
    }
}

impl Image {
    pub fn get_corners(&mut self) -> Vec<usize> {
        let corners = self
            .tiles
            .iter()
            .filter(|tile| match tile.adjacent {
                (None, Some(_), Some(_), None) => true,
                (None, None, Some(_), Some(_)) => true,
                (Some(_), None, None, Some(_)) => true,
                (Some(_), Some(_), None, None) => true,
                _ => false,
            })
            .map(|tile| tile.id)
            .collect::<HashSet<_>>();

        corners.into_iter().collect()
    }

    fn find_adjacent_tiles(&mut self) {
        for i in 0..self.tiles.len() {
            self.tiles[i].adjacent = (
                match self.tiles.iter().find(|adj| {
                    adj.id != self.tiles[i].id && adj.border.bottom == self.tiles[i].border.top
                }) {
                    Some(adj) => Some(adj.id),
                    None => None,
                },
                match self.tiles.iter().find(|adj| {
                    adj.id != self.tiles[i].id && adj.border.left == self.tiles[i].border.right
                }) {
                    Some(adj) => Some(adj.id),
                    None => None,
                },
                match self.tiles.iter().find(|adj| {
                    adj.id != self.tiles[i].id && adj.border.top == self.tiles[i].border.bottom
                }) {
                    Some(adj) => Some(adj.id),
                    None => None,
                },
                match self.tiles.iter().find(|adj| {
                    adj.id != self.tiles[i].id && adj.border.right == self.tiles[i].border.left
                }) {
                    Some(adj) => Some(adj.id),
                    None => None,
                },
            );
        }
    }

    pub fn get_final_image(&mut self) -> Tile {
        let size = self
            .tiles
            .iter()
            .map(|tile| tile.id)
            .collect::<HashSet<_>>()
            .len();
        let row = (size as f64).sqrt() as usize;

        let mut arrangement: HashMap<(usize, usize), &Tile> = HashMap::new();

        for y in 0..row {
            for x in 0..row {
                let tile = match (x, y) {
                    (0, 0) => self
                        .tiles
                        .iter()
                        .find(|tile| match tile.adjacent {
                            (None, Some(_), Some(_), None) => true,
                            _ => false,
                        })
                        .unwrap(),
                    (nx, 0) => {
                        let left = arrangement.get(&(nx - 1, 0)).unwrap();
                        self.tiles
                            .iter()
                            .find(|tile| {
                                tile.id == left.adjacent.1.unwrap()
                                    && tile.adjacent.0.is_none()
                                    && tile.adjacent.3.is_some()
                                    && ((nx == row - 1) ^ tile.adjacent.1.is_some())
                                    && ((nx == 0) ^ tile.adjacent.3.is_some())
                                    && tile.adjacent.3.unwrap() == left.id
                            })
                            .unwrap()
                    }
                    (nx, ny) => {
                        let up = arrangement.get(&(nx, ny - 1)).unwrap();
                        self.tiles
                            .iter()
                            .find(|tile| {
                                tile.id == up.adjacent.2.unwrap()
                                    && tile.adjacent.0.is_some()
                                    && ((nx == row - 1) ^ tile.adjacent.1.is_some())
                                    && ((nx == 0) ^ tile.adjacent.3.is_some())
                                    && ((ny == row - 1) ^ tile.adjacent.2.is_some())
                                    && tile.adjacent.0.unwrap() == up.id
                            })
                            .unwrap()
                    }
                };
                arrangement.insert((x, y), tile);
            }
        }

        let mut pixels: HashMap<Point, Pixel> = HashMap::new();
        let (max_x, max_y) = arrangement
            .values()
            .next()
            .unwrap()
            .pixels
            .keys()
            .max()
            .unwrap();
        let tile_width = (1..*max_x).len();
        let tile_height = (1..*max_y).len();
        for y in 0..row {
            for ty in 1..*max_y {
                for x in 0..row {
                    let tile = arrangement.get(&(x, y)).unwrap();
                    for tx in 1..*max_x {
                        let pixel = tile.pixels.get(&(tx as isize, ty as isize)).unwrap();
                        let x = ((x * tile_width) as isize + tx as isize) as isize - 1;
                        let y = ((y * tile_height) as isize + ty as isize) as isize - 1;
                        pixels.insert((x, y), pixel.clone());
                    }
                }
            }
        }

        Tile {
            id: 0,
            pixels,
            border: Border {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
            },
            adjacent: (None, None, None, None),
        }
    }
}

impl Tile {
    fn flip_y(&self) -> Self {
        let (_, height) = self.pixels.keys().max().unwrap();

        let pixels = self
            .pixels
            .iter()
            .map(|((x, y), pixel)| ((*x, (y * -1) + height), pixel.clone()))
            .collect();

        let border = Border::from_pixels(&pixels);

        Self {
            id: self.id,
            pixels,
            border,
            adjacent: (None, None, None, None),
        }
    }

    fn flip_x(&self) -> Self {
        let (width, _) = self.pixels.keys().max().unwrap();

        let pixels = self
            .pixels
            .iter()
            .map(|((x, y), pixel)| (((x * -1) + width, *y), pixel.clone()))
            .collect();

        let border = Border::from_pixels(&pixels);

        Self {
            id: self.id,
            pixels,
            border,
            adjacent: (None, None, None, None),
        }
    }

    fn rotate_anti_clockwise(&self) -> Self {
        let (width, _) = self.pixels.keys().max().unwrap();

        let pixels = self
            .pixels
            .iter()
            .map(|((x, y), pixel)| ((*y, (x * -1) + width), pixel.clone()))
            .collect();

        Self {
            id: self.id,
            pixels,
            border: Border {
                top: self.border.right,
                right: self.border.bottom,
                bottom: self.border.left,
                left: self.border.top,
            },
            adjacent: (None, None, None, None),
        }
    }

    fn get_sea_monster(&self) -> HashMap<Point, Pixel> {
        SEA_MONSTER
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        (x as isize, y as isize),
                        match c {
                            ' ' => Pixel::White,
                            '#' => Pixel::Black,
                            _ => panic!("Invalid character: {}", c),
                        },
                    )
                })
            })
            .filter(|(_, pixel)| match pixel {
                Pixel::Black => true,
                _ => false,
            })
            .collect()
    }

    pub fn find_sea_monsters(&self) -> usize {
        let sea_monster = self.get_sea_monster();
        let (sea_monster_width, sea_monster_height) = sea_monster.keys().max().unwrap();
        let (max_x, max_y) = self.pixels.keys().max().unwrap();

        let tiles = vec![
            self.clone(),
            self.rotate_anti_clockwise(),
            self.rotate_anti_clockwise().rotate_anti_clockwise(),
            self.rotate_anti_clockwise()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise(),
            self.flip_x(),
            self.flip_x().rotate_anti_clockwise(),
            self.flip_x()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise(),
            self.flip_x()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise(),
            self.flip_y(),
            self.flip_y().rotate_anti_clockwise(),
            self.flip_y()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise(),
            self.flip_y()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise()
                .rotate_anti_clockwise(),
        ];

        for tile in tiles.iter() {
            let monster_count = (0..=(max_y - sea_monster_height))
                .map(|y| {
                    (0..=(max_x - sea_monster_width))
                        .filter(|x| {
                            sea_monster.iter().all(|((sx, sy), _)| {
                                match tile.pixels.get(&(x + sx, y + sy)) {
                                    Some(Pixel::Black) => true,
                                    _ => false,
                                }
                            })
                        })
                        .count()
                })
                .sum::<usize>();
            if monster_count > 0 {
                let total = tile
                    .pixels
                    .iter()
                    .map(|(_, pixel)| match pixel {
                        Pixel::Black => 1,
                        _ => 0,
                    })
                    .sum::<usize>();
                return total - (monster_count * sea_monster.len());
            }
        }
        panic!("Here be no monsters!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r##"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."##;

    #[test]
    fn test_part_one() {
        let mut image = Image::from_str(&INPUT).unwrap();

        assert_eq!(
            20899048083289 as usize,
            image.get_corners().iter().product(),
        );
    }

    #[test]
    fn test_part_two() {
        let mut image = Image::from_str(&INPUT).unwrap();

        let tile = image.get_final_image();

        assert_eq!(273, tile.find_sea_monsters());
    }
}
