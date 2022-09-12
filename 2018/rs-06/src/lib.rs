use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Coordinate = (usize, usize);

fn get_manhattan_distance((xa, ya): &Coordinate, (xb, yb): &Coordinate) -> usize {
    ((*xa as isize - *xb as isize).abs() + (*ya as isize - *yb as isize).abs()) as usize
}

fn get_coordinates(input: &str) -> Vec<Coordinate> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(", ").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect()
}

fn get_dimensions(coordinates: &[Coordinate]) -> Coordinate {
    let (width, _) = coordinates.iter().max_by_key(|&(x, _)| x).unwrap();
    let (_, height) = coordinates.iter().max_by_key(|&(_, y)| y).unwrap();

    (*width, *height)
}

fn get_grid(coordinates: &[Coordinate]) -> HashMap<Coordinate, Option<usize>> {
    let (width, height) = get_dimensions(coordinates);
    (0..=height)
        .flat_map(|y| {
            (0..=width)
                .map(|x| {
                    // For our (x, y) calculate the distance to every coordinate.
                    // Use the enumerated position as an identifier.
                    let mut distances = coordinates
                        .iter()
                        .enumerate()
                        .map(|(i, &(cx, cy))| (i, get_manhattan_distance(&(x, y), &(cx, cy))))
                        .collect::<Vec<_>>();

                    // Sort so the closest coordinates will appear first.
                    distances.sort_unstable_by_key(|(_, distance)| *distance);

                    // If the first two are the same, omit the point.
                    match &distances[..=1] {
                        [(_, first), (_, second)] if *first == *second => ((x, y), None),
                        [(i, _), _] => ((x, y), Some(*i)),
                        _ => panic!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Find the identifiers of those coordinates whose reach extends to
/// the edges.
fn get_ignored(
    grid: &HashMap<Coordinate, Option<usize>>,
    coordinates: &[Coordinate],
) -> HashSet<usize> {
    let (width, height) = get_dimensions(coordinates);
    grid.iter()
        .filter(|&(&(x, y), area)| {
            (x == 0 || x == width || y == 0 || y == height) && area.is_some()
        })
        .map(|(_, area)| match *area {
            Some(a) => a,
            _ => panic!(),
        })
        .collect()
}

pub fn get_part_one(input: &str) -> usize {
    let coordinates = get_coordinates(input);
    let grid = get_grid(&coordinates);
    let ignored = get_ignored(&grid, &coordinates);

    // Skip points not contained in a region or in the ignored list.
    // Track the count of points covered by each coordinate's region,
    // taking the largest value.
    *grid
        .iter()
        .filter(|(_, area)| match area {
            Some(a) => !ignored.contains(a),
            None => false,
        })
        .fold(HashMap::new(), |mut sizes, (_, area)| {
            let area = match area {
                Some(a) => a,
                _ => panic!(),
            };

            *sizes.entry(area).or_insert(0) += 1;

            sizes
        })
        .values()
        .max()
        .unwrap()
}

fn get_grid_within_limit(coordinates: &[Coordinate], limit: usize) -> Vec<Coordinate> {
    let (width, height) = get_dimensions(coordinates);
    (0..=height)
        .flat_map(|y| {
            (0..=width).filter_map(move |x| {
                let distances = coordinates
                    .iter()
                    .enumerate()
                    .map(|(i, &(cx, cy))| (i, get_manhattan_distance(&(x, y), &(cx, cy))))
                    .collect::<Vec<_>>();

                match distances
                    .iter()
                    .map(|(_, distance)| *distance)
                    .sum::<usize>()
                    .cmp(&limit)
                {
                    Ordering::Less => Some((x, y)),
                    Ordering::Equal | Ordering::Greater => None,
                }
            })
        })
        .collect()
}

pub fn get_part_two(input: &str, limit: usize) -> usize {
    let coordinates = get_coordinates(input);
    let grid = get_grid_within_limit(&coordinates, limit);

    grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(17, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(16, get_part_two(INPUT, 32));
    }
}
