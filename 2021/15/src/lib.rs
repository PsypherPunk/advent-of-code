use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn get_cavern(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn get_risk_at((x, y): (usize, usize), cavern: &[Vec<usize>]) -> usize {
    let (y_tile, y_original_tile) = (y / cavern.len(), y % cavern.len());
    let (x_tile, x_original_tile) = (x / cavern[0].len(), x % cavern[0].len());

    (((cavern[y_original_tile][x_original_tile] + y_tile + x_tile) - 1) % 9) + 1
}

fn get_lowest_risk(cavern: Vec<Vec<usize>>, tiles: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    heap.push(Reverse((0_usize, (0_usize, 0_usize))));

    while !heap.is_empty() {
        let (risk, (x, y)) = heap.pop().unwrap().0;

        if (x, y) == ((cavern[0].len() * tiles) - 1, (cavern.len() * tiles) - 1) {
            return risk;
        }

        let positions = [
            (x, y + 1),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x.saturating_sub(1), y),
        ]
        .into_iter()
        .filter(|&(px, py)| {
            !seen.contains(&(px, py))
                && (px, py) != (x, y)
                && px < cavern[0].len() * tiles
                && py < cavern.len() * tiles
        })
        .collect::<HashSet<_>>();

        for (nx, ny) in positions {
            let next_risk = get_risk_at((nx, ny), &cavern);
            heap.push(Reverse((next_risk + risk, (nx, ny))));
            seen.insert((nx, ny));
        }
    }

    unreachable!();
}

pub fn get_part_one(input: &str) -> usize {
    let cavern = get_cavern(input);

    get_lowest_risk(cavern, 1)
}

pub fn get_part_two(input: &str) -> usize {
    let cavern = get_cavern(input);

    get_lowest_risk(cavern, 5)
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    const INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(40, get_part_one(INPUT));
    }

    #[parameterized(position = {
        (27, 2),
        (24, 3),
        (20, 4),
        (38, 6),
        (23, 7),
        (35, 12),
        (11, 16),
        (32, 16),
        (30, 17),
        (17, 20),
        (49, 20),
        (48, 28),
        (36, 34),
        (42, 34),
        (17, 35),
        (20, 35),
        (19, 39),
        (47, 39),
        (7, 45),
        (2, 48),
        (25, 48),
    }, risk = {
        5,
        2,
        9,
        5,
        7,
        5,
        5,
        9,
        7,
        1,
        8,
        8,
        4,
        4,
        5,
        6,
        5,
        3,
        5,
        4,
        9,
    })]
    fn test_tiling(position: (usize, usize), risk: usize) {
        let cavern = get_cavern(INPUT);

        assert_eq!(risk, get_risk_at(position, &cavern));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(315, get_part_two(INPUT));
    }
}
