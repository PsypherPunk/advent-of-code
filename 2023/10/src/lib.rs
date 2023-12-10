use std::cmp::Reverse;
use std::collections::BTreeSet;

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
    let mut stack = std::collections::BinaryHeap::new();
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

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
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

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(4), get_part_one(ONE));
        assert_eq!(Ok(8), get_part_one(TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(ONE));
    }
}
