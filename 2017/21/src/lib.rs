use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

const INITIAL_IMAGE: &str = r#".#.
..#
###"#;

#[derive(Clone, Eq, PartialEq)]
enum PixelState {
    On,
    Off,
}

type Pixel = (usize, usize);

#[derive(Clone, Eq)]
pub struct Image(HashMap<Pixel, PixelState>);

impl Hash for Image {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut items = self.0.iter().collect::<Vec<_>>();
        items.sort_by(|(a, _), (b, _)| a.cmp(&b));
        items.iter().for_each(|((x, y), pixel_state)| {
            x.hash(state);
            y.hash(state);
            match *pixel_state {
                PixelState::On => 1.hash(state),
                PixelState::Off => 0.hash(state),
            }
        });
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .map(|(k, v)| match other.0.get(k) {
                Some(state) => *state == *v,
                None => false,
            })
            .all(|matches| matches)
    }
}

pub struct Rules(HashMap<Image, Image>);

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

impl Image {
    pub fn new() -> Self {
        Image::from_str(&INITIAL_IMAGE).unwrap()
    }

    fn as_string(&self) -> String {
        let (max_x, max_y) = self.0.keys().max().unwrap();
        (0..=*max_y)
            .map(|y| {
                (0..=*max_x)
                    .map(move |x| match self.0.get(&(x, y)) {
                        Some(pixel) => match pixel {
                            PixelState::On => '#',
                            PixelState::Off => '.',
                        },
                        None => panic!(r#"¯\_(ツ)_/¯"#),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_square_at(&self, corner: Pixel, square: usize) -> String {
        let (start_x, start_y) = corner;
        (start_y..(start_y + square))
            .map(|y| {
                (start_x..(start_x + square))
                    .map(move |x| match self.0.get(&(x, y)) {
                        Some(pixel) => match pixel {
                            PixelState::On => '#',
                            PixelState::Off => '.',
                        },
                        None => panic!(r#"¯\_(ツ)_/¯"#),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_flip_x(&self) -> Self {
        Image::from_str(
            &self
                .as_string()
                .lines()
                .map(|line| line.chars().rev().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap()
    }

    fn get_flip_y(&self) -> Self {
        Image::from_str(
            &self
                .as_string()
                .lines()
                .rev()
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap()
    }

    fn get_rotate_90(&self) -> Self {
        let as_str = self.as_string();
        let square = as_str.lines().next().unwrap().len();
        let i_chars = as_str
            .lines()
            .map(|line| line.chars().enumerate().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let rotated = (0..square)
            .map(|nth| {
                i_chars
                    .iter()
                    .rev()
                    .map(|chars| chars.iter().find(|(i, _)| *i == nth).unwrap().1)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        Image::from_str(&rotated).unwrap()
    }

    pub fn get_iteration(&self, rules: &Rules) -> Self {
        let (max_x, max_y) = self.0.keys().max().unwrap();

        let square = match (max_x + 1) % 2 {
            0 => 2,
            _ => 3,
        };
        let replacements = (0..=*max_y)
            .filter(|y| y % square == 0)
            .map(|y| {
                (0..=*max_x)
                    .filter(|x| x % square == 0)
                    .map(move |x| {
                        let sub_square = self.get_square_at((x, y), square);
                        rules
                            .0
                            .get(&Image::from_str(&sub_square).unwrap())
                            .unwrap()
                            .as_string()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let new = replacements
            .iter()
            .map(|row| {
                let len = row.iter().next().unwrap().lines().count();
                (0..len)
                    .map(|nth| {
                        row.iter()
                            .map(|sub| sub.lines().nth(nth).unwrap())
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n");

        Image::from_str(&new).unwrap()
    }

    pub fn get_on_count(&self) -> usize {
        self.0
            .values()
            .filter(|pixel| matches!(pixel, PixelState::On))
            .count()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_string())
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let image = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().map(move |(x, c)| {
                    let state = match c {
                        '#' => PixelState::On,
                        '.' => PixelState::Off,
                        _ => panic!(r#"¯\_(ツ)_/¯"#),
                    };
                    ((x, y), state)
                })
            })
            .collect();

        Ok(Self(image))
    }
}

impl FromStr for Rules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .trim()
            .lines()
            .flat_map(|line| {
                let (source, target) = match line.split(" => ").collect::<Vec<_>>()[..] {
                    [source, target] => (source, target),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                let source_str = source.replace("/", "\n");

                let source = Image::from_str(&source_str).unwrap();
                let target = Image::from_str(&target.replace("/", "\n")).unwrap();

                vec![
                    (source.get_flip_x(), target.clone()),
                    (source.get_flip_y(), target.clone()),
                    (source.get_flip_x().get_flip_y(), target.clone()),
                    (source.get_rotate_90(), target.clone()),
                    (source.get_rotate_90().get_flip_x(), target.clone()),
                    (source.get_rotate_90().get_flip_y(), target.clone()),
                    (
                        source.get_rotate_90().get_flip_x().get_flip_y(),
                        target.clone(),
                    ),
                    (source, target),
                ]
            })
            .collect();

        Ok(Self(rules))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULES: &str = r#"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"#;

    const PART_ONE_FINAL: &str = r#"##.##.
#..#..
......
##.##.
#..#..
......"#;

    #[test]
    fn test_part_one() {
        let rules = Rules::from_str(&RULES).unwrap();
        let mut image = Image::new();

        for _ in 0..2 {
            image = image.get_iteration(&rules);
        }

        assert_eq!(PART_ONE_FINAL, image.as_string());
    }
}
