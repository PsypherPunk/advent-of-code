use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

type Point = (isize, isize);

const SEA_MONSTER: &str = r##"                  #
#    ##    ##    ###
 #  #  #  #  #  #"##;

#[derive(Clone)]
enum Pixel {
    White,
    Black,
}

#[derive(Clone)]
struct Border {
    top: String,
    right: String,
    bottom: String,
    left: String,
}

#[derive(Clone)]
pub struct Tile {
    pub id: usize,
    pixels: HashMap<Point, Pixel>,
    border: Border,
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
            .map(|x| pixels.get(&(x, 0)).unwrap().to_string())
            .collect::<String>();
        let right = (0..=*max_y)
            .map(|y| pixels.get(&(*max_x, y)).unwrap().to_string())
            .collect::<String>();
        let bottom = (0..=*max_x)
            .map(|x| pixels.get(&(x, *max_y)).unwrap().to_string())
            .collect::<String>();
        let left = (0..=*max_y)
            .map(|y| pixels.get(&(0, y)).unwrap().to_string())
            .collect::<String>();

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

        Ok(Self { id, pixels, border })
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

        Ok(Image { tiles })
    }
}

impl Image {
    #[allow(clippy::suspicious_operation_groupings)]
    pub fn get_corners(&mut self) -> Vec<usize> {
        let corners =
            self.tiles
                .iter()
                .filter(|tile| {
                    !self
                        .tiles
                        .iter()
                        .any(|other| tile.id != other.id && tile.border.top == other.border.bottom)
                        && !self.tiles.iter().any(|other| {
                            tile.id != other.id && tile.border.left == other.border.right
                        })
                })
                .map(|tile| tile.id)
                .collect::<HashSet<_>>();

        corners.into_iter().collect()
    }

    #[allow(clippy::suspicious_operation_groupings)]
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
                        .find(|tile| {
                            !self.tiles.iter().any(|other| {
                                tile.id != other.id && tile.border.top == other.border.bottom
                            }) && !self.tiles.iter().any(|other| {
                                tile.id != other.id && tile.border.left == other.border.right
                            })
                        })
                        .unwrap(),
                    (nx, 0) => {
                        let left = arrangement.get(&(nx - 1, 0)).unwrap();
                        self.tiles
                            .iter()
                            .find(|tile| {
                                left.id != tile.id && tile.border.left == left.border.right
                            })
                            .unwrap()
                    }
                    (nx, ny) => {
                        let up = arrangement.get(&(nx, ny - 1)).unwrap();
                        self.tiles
                            .iter()
                            .find(|tile| up.id != tile.id && tile.border.top == up.border.bottom)
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
                top: "".to_string(),
                right: "".to_string(),
                bottom: "".to_string(),
                left: "".to_string(),
            },
        }
    }
}

impl Tile {
    fn flip_y(&self) -> Tile {
        let (_, height) = self.pixels.keys().max().unwrap();

        let pixels = self
            .pixels
            .iter()
            .map(|((x, y), pixel)| ((*x, (y * -1) + height), pixel.clone()))
            .collect();

        let border = Border::from_pixels(&pixels);

        Tile {
            id: self.id,
            pixels,
            border,
        }
    }

    fn flip_x(&self) -> Tile {
        let (width, _) = self.pixels.keys().max().unwrap();

        let pixels = self
            .pixels
            .iter()
            .map(|((x, y), pixel)| (((x * -1) + width, *y), pixel.clone()))
            .collect();

        let border = Border::from_pixels(&pixels);

        Tile {
            id: self.id,
            pixels,
            border,
        }
    }

    fn rotate_anti_clockwise(&self) -> Tile {
        let (width, _) = self.pixels.keys().max().unwrap();

        let pixels = self
            .pixels
            .iter()
            .map(|((x, y), pixel)| ((*y, (x * -1) + width), pixel.clone()))
            .collect();

        let border = Border::from_pixels(&pixels);

        Tile {
            id: self.id,
            pixels,
            border,
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
            .filter(|(_, pixel)| matches!(pixel, Pixel::Black))
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
                                matches!(tile.pixels.get(&(x + sx, y + sy)), Some(Pixel::Black))
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
