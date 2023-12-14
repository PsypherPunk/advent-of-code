use std::collections::BTreeMap;

// TODO: this is horrible; do better.
fn tilt_north(positions: &mut Vec<Vec<u8>>) {
    let mut done = false;

    while !done {
        done = true;
        for y in 0..positions.len() - 1 {
            for x in 0..positions[0].len() {
                if positions[y + 1][x] == b'O' && positions[y][x] == b'.' {
                    positions[y][x] = b'O';
                    positions[y + 1][x] = b'.';
                    done = false;
                }
            }
        }
    }
}

fn rotate_clockwise(positions: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut rotated = vec![vec![0; positions.len()]; positions[0].len()];

    for y in 0..positions.len() {
        for (x, row) in rotated.iter_mut().enumerate().take(positions[0].len()) {
            row[positions.len() - 1 - y] = positions[y][x];
        }
    }

    rotated
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
            positions = rotate_clockwise(&positions);
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
