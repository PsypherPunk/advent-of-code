use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, VecDeque};

#[derive(Debug, Default)]
struct Tile {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    start: bool,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            'F' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            '.' => Default::default(),
            'S' => Self {
                start: true,
                ..Default::default()
            },
            _ => panic!("invalid char: {}", value),
        }
    }
}

// TODO: this is crap.
pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut sketch: Vec<Vec<Tile>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.into()).collect())
        .collect();

    let start = (0..sketch.len())
        .find_map(|y| (0..sketch[y].len()).find_map(|x| (sketch[y][x].start).then_some((y, x))))
        .ok_or(format!("no start: {}", input))?;

    sketch[start.0][start.1].north = start.0 > 0 && sketch[start.0 - 1][start.1].south;
    sketch[start.0][start.1].south =
        start.0 < sketch.len() - 1 && sketch[start.0 + 1][start.1].north;
    sketch[start.0][start.1].west = start.1 > 0 && sketch[start.0][start.1 - 1].east;
    sketch[start.0][start.1].east =
        start.1 < sketch[start.0].len() - 1 && sketch[start.0][start.1 + 1].west;

    let mut distance = 0;
    let mut stack = BinaryHeap::new();
    let mut discovered: BTreeSet<(usize, usize)> = BTreeSet::new();
    stack.push((Reverse(0), start));
    while let Some((Reverse(current_distance), (y, x))) = stack.pop() {
        if !discovered.insert((y, x)) {
            continue;
        }
        distance = distance.max(current_distance);
        let tile = &sketch[y][x];
        if tile.north {
            stack.push((Reverse(current_distance + 1), (y - 1, x)));
        }
        if tile.south {
            stack.push((Reverse(current_distance + 1), (y + 1, x)));
        }
        if tile.west {
            stack.push((Reverse(current_distance + 1), (y, x - 1)));
        }
        if tile.east {
            stack.push((Reverse(current_distance + 1), (y, x + 1)));
        }
    }

    Ok(distance)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut sketch: Vec<Vec<Tile>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.into()).collect())
        .collect();

    let start = (0..sketch.len())
        .find_map(|y| (0..sketch[y].len()).find_map(|x| (sketch[y][x].start).then_some((y, x))))
        .ok_or(format!("no start: {}", input))?;

    sketch[start.0][start.1].north = start.0 > 0 && sketch[start.0 - 1][start.1].south;
    sketch[start.0][start.1].south =
        start.0 < sketch.len() - 1 && sketch[start.0 + 1][start.1].north;
    sketch[start.0][start.1].west = start.1 > 0 && sketch[start.0][start.1 - 1].east;
    sketch[start.0][start.1].east =
        start.1 < sketch[start.0].len() - 1 && sketch[start.0][start.1 + 1].west;

    let mut stack = BinaryHeap::new();
    let mut discovered = BTreeSet::new();
    stack.push((Reverse(0), start));
    while let Some((Reverse(current_distance), (y, x))) = stack.pop() {
        if !discovered.insert((y, x)) {
            continue;
        }
        let tile = &sketch[y][x];
        if tile.north {
            stack.push((Reverse(current_distance + 1), (y - 1, x)));
        }
        if tile.south {
            stack.push((Reverse(current_distance + 1), (y + 1, x)));
        }
        if tile.west {
            stack.push((Reverse(current_distance + 1), (y, x - 1)));
        }
        if tile.east {
            stack.push((Reverse(current_distance + 1), (y, x + 1)));
        }
    }

    let s = match (
        sketch[start.0][start.1].north,
        sketch[start.0][start.1].east,
        sketch[start.0][start.1].south,
        sketch[start.0][start.1].west,
    ) {
        (true, true, false, false) => "L",
        (true, false, true, false) => "|",
        (false, true, false, true) => "-",
        (true, false, false, true) => "J",
        (false, false, true, true) => "7",
        (false, true, true, false) => "F",
        _ => unreachable!(),
    };

    let enclosed = input
        .replace("S", s)
        .trim()
        .lines()
        .enumerate()
        .map(|(y, row)| {
            let discovered = &discovered;
            let mut stack = VecDeque::new();

            row.char_indices()
                .map(move |(x, c)| match discovered.contains(&(y, x)) {
                    false => stack.len() % 2,
                    true => {
                        match c {
                            '|' => {
                                stack.push_front(c);
                            }
                            'J' => {
                                match stack.iter().peekable().peek() {
                                    None => {}
                                    Some('F') => {
                                        _ = stack.pop_front();
                                        stack.push_front('|');
                                    }
                                    Some('|') => {}
                                    _ => {
                                        _ = stack.pop_front();
                                    }
                                };
                            }
                            '7' => match stack.iter().peekable().peek() {
                                None => {}
                                Some('L') => {
                                    _ = stack.pop_front();
                                    stack.push_front('|');
                                }
                                Some('|') => {}
                                _ => {
                                    _ = stack.pop_front();
                                }
                            },
                            'L' | 'F' => {
                                stack.push_front(c);
                            }
                            _ => {}
                        };

                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    Ok(enclosed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;
    const TWO: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;
    const THREE: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
    const FOUR: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
    const FIVE: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(4), get_part_one(ONE));
        assert_eq!(Ok(8), get_part_one(TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(4), get_part_two(THREE));
        assert_eq!(Ok(8), get_part_two(FOUR));
        assert_eq!(Ok(10), get_part_two(FIVE));
    }
}
