use std::collections::HashMap;
use std::str::FromStr;

type Pixel = (isize, isize);

struct Image {
    min: Pixel,
    max: Pixel,
    pixels: HashMap<Pixel, bool>,
    default: bool,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c == '#'))
            })
            .collect::<HashMap<_, _>>();

        let min = pixels.keys().min().unwrap();
        let max = pixels.keys().max().unwrap();

        Ok(Image {
            min: *min,
            max: *max,
            pixels,
            default: false,
        })
    }
}

impl Image {
    fn get_enhanced_image(&self, enhancement: &[bool]) -> Self {
        let min = (self.min.0 - 1, self.min.1 - 1);
        let max = (self.max.0 + 1, self.max.1 + 1);

        let pixels = (min.1..=max.1)
            .flat_map(|y| {
                (min.0..=max.0).map(move |x| {
                    let lookup = [
                        (x - 1, y - 1),
                        (x, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y),
                        (x, y),
                        (x + 1, y),
                        (x - 1, y + 1),
                        (x, y + 1),
                        (x + 1, y + 1),
                    ]
                    .iter()
                    .map(|pixel| match self.pixels.get(pixel) {
                        Some(light) => *light,
                        None => self.default,
                    })
                    .fold(0, |acc, bit| acc << 1 | bit as usize);

                    ((x, y), enhancement[lookup])
                })
            })
            .collect();

        let default = !self.default;
        Image {
            min,
            max,
            pixels,
            default,
        }
    }
}

pub fn get_part_one(input: &str) -> usize {
    let (enhancement, image) = input.trim().split_once("\n\n").unwrap();

    let enhancement = enhancement.chars().map(|c| c == '#').collect::<Vec<_>>();
    assert_eq!(512, enhancement.len());

    let mut image = Image::from_str(image).unwrap();

    for _ in 0..2 {
        image = image.get_enhanced_image(&enhancement);
    }

    image.pixels.values().filter(|&pixel| *pixel).count()
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(35, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
