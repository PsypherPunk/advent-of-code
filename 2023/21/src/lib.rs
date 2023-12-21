use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
fn to_string(garden: &[Vec<u8>]) -> String {
    garden
        .iter()
        .map(|row| row.iter().map(|b| *b as char).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn bfs(garden: &[Vec<u8>], start: (isize, isize), target: isize) -> isize {
    let mut queue = VecDeque::from_iter([(start, 0)]);
    let mut seen = HashSet::new();

    let mut plots = HashSet::new();

    let ylen = garden.len() as isize;
    let xlen = garden[0].len() as isize;

    while let Some(((x, y), steps)) = queue.pop_front() {
        if steps == target {
            plots.insert((x, y));
            continue;
        }

        if seen.contains(&((x, y), steps)) {
            continue;
        }
        seen.insert(((x, y), steps));

        let next = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .into_iter()
            .filter_map(|(nx, ny)| {
                let effective_y = ny.rem_euclid(ylen) as usize;
                let effective_x = nx.rem_euclid(xlen) as usize;

                match garden[effective_y][effective_x] {
                    b'#' => None,
                    _ => Some(((nx, ny), steps + 1)),
                }
            })
            .collect::<Vec<_>>();

        queue.extend(next);
    }

    plots.len() as isize
}

fn lagrange(a: isize, b: isize, c: isize) -> (isize, isize, isize) {
    (a / 2 - b + c / 2, -3 * (a / 2) + 2 * b - c / 2, a)
}

pub fn get_part_one(input: &str, target: isize) -> Result<isize, String> {
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
                        start = (x as isize, y as isize);
                        b'.'
                    }
                    p => *p,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(bfs(&garden, start, target))
}

pub fn get_part_two(input: &str, target: isize) -> Result<isize, String> {
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
                        start = (x as isize, y as isize);
                        b'.'
                    }
                    p => *p,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let steps_to_bounday = ((garden.len() - 1) / 2) as isize;
    let len = garden.len() as isize;
    let values = (
        bfs(&garden, start, steps_to_bounday),
        bfs(&garden, start, steps_to_bounday + len),
        bfs(&garden, start, steps_to_bounday + len * 2),
    );
    let polynomial = lagrange(values.0, values.1, values.2);
    let target = (target - steps_to_bounday) / len;

    Ok(polynomial.0 * target * target + polynomial.1 * target + polynomial.2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

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

    #[parameterized(steps = {
        6, 10, 50, 100, 500, 1000, 5000
    }, plots = {
        16, 50, 1594, 6536, 167004, 668697, 16733044
    })]
    fn test_part_two(steps: isize, plots: isize) {
        // TODO: none of these pass…?
        assert_eq!(Ok(plots), get_part_two(INPUT, steps));
    }
}
