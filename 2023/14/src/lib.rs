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

fn total_load(positions: Vec<Vec<u8>>) -> usize {
    let height = positions.len();

    positions
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .filter_map(move |&column| match column {
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

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
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
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
