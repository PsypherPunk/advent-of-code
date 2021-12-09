fn get_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x + 1, y),
        (x, y + 1),
    ]
}

pub fn get_part_one(input: &str) -> usize {
    let lava_tubes = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = lava_tubes[0].len();
    let height = lava_tubes.len();

    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if get_neighbours(x, y)
                .iter()
                .filter(|(nx, ny)| (*nx, *ny) != (x, y) && *nx < width && *ny < height)
                .all(|&(nx, ny)| lava_tubes[ny][nx] > lava_tubes[y][x])
            {
                low_points.push((x, y));
            }
        }
    }

    low_points.iter().map(|&(x, y)| lava_tubes[y][x] + 1).sum()
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
        assert_eq!(15, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, 2)
    }
}
