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

pub fn get_part_one(input: &str) -> usize {
    let cavern = get_cavern(input);

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    heap.push(Reverse((0_usize, (0_usize, 0_usize))));

    while !heap.is_empty() {
        let (risk, (x, y)) = heap.pop().unwrap().0;

        if (x, y) == (cavern[0].len() - 1, cavern.len() - 1) {
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
                && px < cavern[0].len()
                && py < cavern.len()
        })
        .collect::<HashSet<_>>();

        for (nx, ny) in positions {
            heap.push(Reverse((cavern[ny][nx] + risk, (nx, ny))));
            seen.insert((nx, ny));
        }
    }

    unreachable!();
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
