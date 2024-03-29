use std::collections::{HashSet, VecDeque};

fn get_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y + 1),
    ]
}

fn get_lava_tubes(input: &str) -> Result<Vec<Vec<usize>>, String> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|digit| digit as usize))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "invalid digit".to_owned())
}

fn get_low_points(lava_tubes: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let width = lava_tubes[0].len();
    let height = lava_tubes.len();

    (0..height)
        .flat_map(|y| {
            (0..width)
                .map(move |x| {
                    (x, y)
                })
        })
        .filter(|&(x, y)| {
            get_neighbours(x, y)
                .iter()
                .filter(|(nx, ny)| (*nx, *ny) != (x, y) && *nx < width && *ny < height)
                .all(|&(nx, ny)| lava_tubes[ny][nx] > lava_tubes[y][x])
        })
        .collect()
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let lava_tubes = get_lava_tubes(input)?;
    let low_points = get_low_points(&lava_tubes);

    let sum = low_points.iter().map(|&(x, y)| lava_tubes[y][x] + 1).sum();

    Ok(sum)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let lava_tubes = get_lava_tubes(input)?;
    let low_points = get_low_points(&lava_tubes);
    let width = lava_tubes[0].len();
    let height = lava_tubes.len();

    let mut all_seen: HashSet<(usize, usize)> = HashSet::new();
    let mut basin_sizes = Vec::new();

    for low_point in low_points {
        if all_seen.contains(&low_point) {
            continue;
        }
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(low_point);

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            seen.insert((x, y));
            let neighbours = get_neighbours(x, y)
                .into_iter()
                .filter(|(nx, ny)| (*nx, *ny) != (x, y) && *nx < width && *ny < height)
                .collect::<Vec<_>>();
            for (nx, ny) in neighbours {
                if lava_tubes[ny][nx] != 9 && !seen.contains(&(nx, ny)) {
                    queue.push_back((nx, ny));
                }
            }
        }
        basin_sizes.push(seen.len());
        all_seen.extend(seen.iter());
    }

    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    let product = basin_sizes.iter().take(3).product();

    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(15, get_part_one(INPUT).unwrap());
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1134, get_part_two(INPUT).unwrap());
    }
}
