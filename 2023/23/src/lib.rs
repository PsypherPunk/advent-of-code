// TODO: ditch pathfinding; more trouble than its worth.
use pathfinding::prelude::{bfs, Grid, Matrix};
use std::collections::{HashMap, HashSet};

// TODO: this is horrid.
type PathDistances = HashMap<(usize, usize), HashSet<((usize, usize), usize)>>;

fn go_hiking(
    trails: &Matrix<&u8>,
    tile: (usize, usize),
    distance: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> usize {
    if tile.0 == trails.rows - 1 {
        return distance;
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
        .unwrap_or(0);

    seen.remove(&tile);

    distance
}

fn go_hiking_again(
    trails: &Grid,
    proximity: &PathDistances,
    tile: (usize, usize),
    distance: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> usize {
    if tile.1 == trails.height - 1 {
        return distance;
    }

    seen.insert(tile);

    let distance = proximity[&tile]
        .iter()
        .filter(|(n, _)| !seen.contains(n))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(neighbour, c)| go_hiking_again(trails, proximity, *neighbour, distance + c, seen))
        .max()
        .unwrap_or(0);

    seen.remove(&tile);

    distance
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let trails = input
        .trim()
        .lines()
        .map(str::as_bytes)
        .collect::<Matrix<_>>();

    let hike = go_hiking(&trails, (0, 1), 0, &mut HashSet::new());

    Ok(hike)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let trails = input
        .trim()
        .lines()
        .map(str::as_bytes)
        .collect::<Matrix<_>>();
    let trails: Grid = trails.map(|b| *b != b'#').into();

    let junctions = trails
        .iter()
        .filter(|&n| trails.neighbours(n).len() != 2)
        .collect::<HashSet<_>>();

    let mut neighbours: PathDistances = HashMap::new();

    for &junction in &junctions {
        for next in trails.neighbours(junction) {
            neighbours.entry(junction).or_default().insert((next, 1));
            neighbours.entry(next).or_default().insert((junction, 1));
            if let Some(path) = bfs(
                &next,
                |tile| {
                    trails
                        .neighbours(*tile)
                        .into_iter()
                        .filter(|n| !junctions.contains(n))
                },
                |tile| {
                    *tile != next
                        && trails
                            .neighbours(*tile)
                            .into_iter()
                            .any(|tile| junctions.contains(&tile))
                },
            ) {
                let end = path
                    .last()
                    .ok_or(format!("couldn't find end of path: {:?}", path))?;
                let distance = path.len() - 1;
                neighbours.entry(next).or_default().insert((*end, distance));
            }
        }
    }
    let hike = go_hiking_again(&trails, &neighbours, (1, 0), 0, &mut HashSet::new());

    Ok(hike)
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
        assert_eq!(Ok(154), get_part_two(INPUT));
    }
}
