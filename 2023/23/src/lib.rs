use pathfinding::prelude::Matrix;
use std::collections::HashSet;

fn go_hiking(
    trails: &Matrix<&u8>,
    tile: (usize, usize),
    distance: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> Result<usize, String> {
    if tile.0 == trails.rows - 1 {
        return Ok(distance);
    }

    seen.insert(tile);

    let neighbours = match trails[tile] {
        b'>' => vec![(tile.0, tile.1 + 1)],
        b'<' => vec![(tile.0, tile.1 - 1)],
        b'v' => vec![(tile.0 + 1, tile.1)],
        b'^' => vec![(tile.0 - 1, tile.1)],
        _ => trails.neighbours(tile, false).collect::<Vec<_>>(),
    }
    .into_iter()
    .filter(|n| *trails[*n] != b'#' && !seen.contains(n))
    .collect::<Vec<_>>();

    let distance = neighbours
        .into_iter()
        .map(|neighbour| go_hiking(trails, neighbour, distance + 1, seen))
        .max()
        .unwrap_or(Ok(0));

    seen.remove(&tile);

    distance
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let trails = input
        .trim()
        .lines()
        .map(str::as_bytes)
        .collect::<Matrix<_>>();

    let hike = go_hiking(&trails, (0, 1), 0, &mut HashSet::new())?;

    Ok(hike)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(94), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
