use std::collections::BTreeMap;
use std::mem;

fn tilt_north(positions: &mut Vec<Vec<u8>>) {
    (0..(positions.len() - 1)).for_each(|y| {
        (0..positions[0].len()).for_each(|x| {
            if positions[y][x] == b'.'
                && (y == 0 || (positions[y - 1][x] == b'#' || positions[y - 1][x] == b'O'))
            {
                if let Some((ty, b'O')) =
                    ((y + 1)..positions.len()).find_map(|ty| match positions[ty][x] {
                        b'#' => Some((ty, b'#')),
                        b'O' => Some((ty, b'O')),
                        _ => None,
                    })
                {
                    let (a, b) = positions.split_at_mut(ty);
                    mem::swap(&mut a[y][x], &mut b[0][x]);
                }
            }
        });
    });
}

fn rotate_clockwise(positions: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..positions[0].len())
        .map(|x| positions.iter().rev().map(|row| row[x]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[allow(unused)]
fn display(positions: &[Vec<u8>]) -> String {
    positions
        .iter()
        .map(|row| row.iter().map(|b| *b as char).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn total_load(positions: Vec<Vec<u8>>) -> usize {
    let height = positions.len();

    positions
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().filter_map(move |&column| match column {
                b'O' => Some(height - y),
                _ => None,
            })
        })
        .sum()
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut positions = input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    tilt_north(&mut positions);

    Ok(total_load(positions))
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut positions = input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut seen = BTreeMap::new();
    for cycle in 1.. {
        for _ in 0..4 {
            tilt_north(&mut positions);
            positions = rotate_clockwise(positions);
        }
        if let Some(seen_cycle) = seen.insert(positions.clone(), cycle) {
            if (1000000000 - cycle) % (cycle - seen_cycle) == 0 {
                break;
            }
        }
    }

    Ok(total_load(positions))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(136), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(64), get_part_two(INPUT));
    }
}
