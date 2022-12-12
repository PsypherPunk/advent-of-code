use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidHeightMapError,
}

struct HeightMap {
    elevations: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn get_steps(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        [
            (x, y.saturating_sub(1)),
            ((x + 1).min(self.width - 1), y),
            (x, (y + 1).min(self.height - 1)),
            (x.saturating_sub(1), y),
        ]
        .into_iter()
        .filter(|(dx, dy)| {
            (dx, dy) != (&x, &y)
                && ((self.elevations[*dy][*dx] as usize) < (self.elevations[y][x] as usize)
                    || (self.elevations[*dy][*dx] as usize)
                        .abs_diff(self.elevations[y][x] as usize)
                        <= 1)
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
    }
}

fn get_heightmap(input: &str) -> Result<HeightMap, AdventOfCodeError> {
    let mut start = None;
    let mut end = None;
    let mut width = None;
    let elevations: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            width = Some(line.len());
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Some((x, y));
                        'a'
                    }
                    'E' => {
                        end = Some((x, y));
                        'z'
                    }
                    elevation => elevation,
                })
                .collect()
        })
        .collect();
    let height = elevations.len();

    Ok(HeightMap {
        elevations,
        width: width.ok_or(AdventOfCodeError::InvalidHeightMapError)?,
        height,
        start: start.ok_or(AdventOfCodeError::InvalidHeightMapError)?,
        end: end.ok_or(AdventOfCodeError::InvalidHeightMapError)?,
    })
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let heightmap = get_heightmap(input)?;

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((heightmap.start, 0));
    seen.insert(heightmap.start);
    let mut fewest_steps = None;

    while let Some((current, distance)) = queue.pop_front() {
        if current == heightmap.end {
            fewest_steps = Some(distance);
        }

        for step in heightmap.get_steps(current) {
            if !seen.contains(&step) {
                queue.push_back((step, distance + 1));
                seen.insert(step);
            }
        }
    }

    fewest_steps.ok_or(AdventOfCodeError::InvalidHeightMapError)
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(31), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
