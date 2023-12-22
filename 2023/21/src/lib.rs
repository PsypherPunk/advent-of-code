use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
fn to_string(garden: &[Vec<u8>]) -> String {
    garden
        .iter()
        .map(|row| row.iter().map(|b| *b as char).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn bfs(garden: &[Vec<u8>], start: (isize, isize), target: isize) -> HashSet<(isize, isize)> {
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

    plots
}

fn distances(garden: &[Vec<u8>], start: (isize, isize)) -> HashMap<(isize, isize), isize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    let ylen = garden.len() as isize;
    let xlen = garden[0].len() as isize;

    while let Some(((x, y), steps)) = queue.pop_front() {
        if distances.contains_key(&(x, y)) {
            continue;
        }

        distances.insert((x, y), steps);

        let next = [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .into_iter()
            .filter_map(|(nx, ny)| {
                let effective_y = ny.rem_euclid(ylen) as usize;
                let effective_x = nx.rem_euclid(xlen) as usize;

                match garden[effective_y][effective_x] {
                    b'#' => None,
                    _ => Some(((effective_x as isize, effective_y as isize), steps + 1)),
                }
            })
            .collect::<Vec<_>>();

        queue.extend(next);
    }

    distances
}

pub fn get_part_one(input: &str, target: isize) -> Result<usize, String> {
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

    Ok(bfs(&garden, start, target).len())
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

    let distances = distances(&garden, start);

    let (odd, even, odd_edges, even_edges) = distances.iter().fold(
        (0, 0, 0, 0),
        |(odd, even, odd_edges, even_edges), (_, distance)| {
            if *distance % 2 == 1 && *distance > 65 {
                (odd + 1, even, odd_edges + 1, even_edges)
            } else if *distance % 2 == 1 {
                (odd + 1, even, odd_edges, even_edges)
            } else if *distance % 2 == 0 && *distance > 65 {
                (odd, even + 1, odd_edges, even_edges + 1)
            } else {
                (odd, even + 1, odd_edges, even_edges)
            }
        },
    );
    let steps_to_bounday = ((garden.len() - 1) / 2) as isize;
    let len = garden.len() as isize;
    let count = (target - steps_to_bounday) / len;
    let total_odd = odd * (count + 1) * (count + 1);
    let total_even = even * (count * count);
    let total_odd_edges = odd_edges * (count + 1);
    let total_even_edges = count * even_edges;

    Ok(total_odd + total_even - total_odd_edges + total_even_edges)
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
