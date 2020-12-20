use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

type Point = (isize, isize);

#[derive(Clone)]
enum Pixel {
    White,
    Black,
}

struct Border {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

struct Tile {
    id: usize,
    pixels: HashMap<Point, Pixel>,
    border: Border,
}

pub struct Image {
    tiles: Vec<Tile>,
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
                    Pixel::White => 1,
                    Pixel::Black => 0,
                };
                bit << index
            })
            .sum();
        let right = (0..=*max_y)
            .enumerate()
            .map(|(index, y)| {
                let bit = match pixels.get(&(*max_x, y)).unwrap() {
                    Pixel::White => 1,
                    Pixel::Black => 0,
                };
                bit << index
            })
            .sum();
        let bottom = (0..=*max_x)
            .rev()
            .enumerate()
            .map(|(index, x)| {
                let bit = match pixels.get(&(x, *max_y)).unwrap() {
                    Pixel::White => 1,
                    Pixel::Black => 0,
                };
                bit << index
            })
            .sum();
        let left = (0..=*max_y)
            .rev()
            .enumerate()
            .map(|(index, y)| {
                let bit = match pixels.get(&(0, y)).unwrap() {
                    Pixel::White => 1,
                    Pixel::Black => 0,
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

        Ok(Self { id, pixels, border })
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s.trim().split("\n\n").collect::<Vec<_>>();

        Ok(Self {
            tiles: tiles
                .iter()
                .map(|tile| Tile::from_str(tile).unwrap())
                .collect(),
        })
    }
}

impl Image {
    pub fn get_borders(&self) -> Vec<usize> {
        let all_borders = self
            .tiles
            .iter()
            .flat_map(|tile| {
                let flipped_y = tile.flip_y();
                let flipped_x = tile.flip_x();

                vec![
                    tile.border.top,
                    tile.border.right,
                    tile.border.bottom,
                    tile.border.left,
                    flipped_y.border.top,
                    flipped_y.border.bottom,
                    flipped_x.border.left,
                    flipped_x.border.right,
                ]
            })
            .collect::<Vec<_>>();

        self.tiles
            .iter()
            .filter(|tile| {
                vec![
                    all_borders
                        .iter()
                        .filter(|&border| *border == tile.border.top)
                        .count(),
                    all_borders
                        .iter()
                        .filter(|&border| *border == tile.border.right)
                        .count(),
                    all_borders
                        .iter()
                        .filter(|&border| *border == tile.border.bottom)
                        .count(),
                    all_borders
                        .iter()
                        .filter(|&border| *border == tile.border.left)
                        .count(),
                ]
                .iter()
                .filter(|&count| *count == 1)
                .count()
                    == 2
            })
            .map(|tile| tile.id)
            .collect()
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
        }
    }

    #[allow(dead_code)]
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
        }
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
        let image = Image::from_str(&INPUT).unwrap();

        assert_eq!(
            20899048083289 as usize,
            image.get_borders().iter().product(),
        );
    }
}
