use std::collections::{BTreeMap, BinaryHeap};

fn dfs(map: Vec<Vec<u8>>, min_blocks: isize, max_blocks: isize) -> isize {
    let mut stack = BinaryHeap::new();
    let mut costs = BTreeMap::new();

    stack.push((0, (0, 0, (0, 0))));

    while let Some((heat_loss, (y, x, direction))) = stack.pop() {
        if (y, x) == (map.len() - 1, map[0].len() - 1) {
            return -heat_loss;
        }

        if costs
            .get(&(y, x, direction))
            .is_some_and(|&cost| -heat_loss > cost)
        {
            continue;
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if direction == (dy, dx) || direction == (-dy, -dx) {
                continue;
            }
            let mut next_cost = -heat_loss;
            for block in 1..=max_blocks {
                let ny = (y as isize + dy * block) as usize;
                let nx = (x as isize + dx * block) as usize;
                if ny >= map.len() || nx >= map[0].len() {
                    continue;
                }
                next_cost += (map[ny][nx]) as isize;
                let key = (ny, nx, (dy, dx));
                if min_blocks <= block
                    && next_cost < *costs.get(&key).unwrap_or(&(map.len().pow(2) as isize))
                {
                    costs.insert(key, next_cost);
                    stack.push((-next_cost, key));
                }
            }
        }
    }

    panic!("failed to determine minimum heat loss")
}

pub fn get_part_one(input: &str) -> Result<isize, String> {
    let map = input
        .trim()
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| b - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(dfs(map, 1, 3))
}

pub fn get_part_two(input: &str) -> Result<isize, String> {
    let map = input
        .trim()
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| b - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(dfs(map, 4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;
    const TWO: &str = r#"111111111111
999999999991
999999999991
999999999991
999999999991
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(102), get_part_one(ONE));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(94), get_part_two(ONE));
        assert_eq!(Ok(71), get_part_two(TWO));
    }
}
