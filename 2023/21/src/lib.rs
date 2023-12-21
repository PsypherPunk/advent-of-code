use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
fn to_string(garden: &[Vec<u8>]) -> String {
    garden
        .iter()
        .map(|row| row.iter().map(|b| *b as char).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn bfs(garden: Vec<Vec<u8>>, start: (usize, usize), target: usize) -> usize {
    let mut queue = VecDeque::from_iter([(start, 0)]);
    let mut seen = HashSet::new();

    let mut plots = HashSet::new();

    while let Some(((x, y), steps)) = queue.pop_front() {
        if steps == target {
            plots.insert((x, y));
            continue;
        }

        if seen.contains(&((x, y), steps)) {
            continue;
        }
        seen.insert(((x, y), steps));

        let next = [
            (x, y.saturating_sub(1)),
            (x, y + 1),
            (x.saturating_sub(1), y),
            (x + 1, y),
        ]
        .iter()
        .filter(|(nx, ny)| {
            *nx < garden[0].len()
                && *ny < garden.len()
                && (*nx, *ny) != (x, y)
                && garden[*ny][*nx] != b'#'
        })
        .map(|(nx, ny)| ((*nx, *ny), steps + 1))
        .collect::<Vec<_>>();

        queue.extend(next);
    }

    plots.len()
}

pub fn get_part_one(input: &str, target: usize) -> Result<usize, String> {
    let mut start = (0, 0);
    let garden = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, position)| match position {
                    b'S' => {
                        start = (x, y);
                        b'.'
                    }
                    p => *p,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(bfs(garden, start, target))
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(16), get_part_one(INPUT, 6));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
