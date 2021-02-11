use std::collections::HashSet;

mod knot;

fn get_grid(key: &str) -> Vec<String> {
    (0..128)
        .map(|row| {
            let mut knot_hash = knot::KnotHash::new(0, 255);
            let input = format!("{}-{}", key, row);

            knot_hash.binary(&input)
        })
        .collect()
}

fn get_ones(key: &str) -> HashSet<(usize, usize)> {
    let grid = get_grid(&key);

    grid.iter()
        .enumerate()
        .flat_map(|(y, binary)| {
            binary
                .chars()
                .enumerate()
                .filter(|(_, b)| *b == '1')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn get_neighbours((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;

    vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
        .iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|&(x, y)| (x as usize, y as usize))
        .collect()
}

fn depth_first_search(squares: &mut HashSet<(usize, usize)>, square: (usize, usize)) {
    squares.remove(&square);
    for neighbour in get_neighbours(square) {
        if squares.contains(&neighbour) {
            depth_first_search(squares, neighbour);
        }
    }
}

pub fn get_used_square_count(key: &str) -> usize {
    get_ones(&key).len()
}

pub fn get_region_count(key: &str) -> usize {
    let mut ones = get_ones(&key);

    let mut count = 0;
    while let Some(&square) = ones.iter().next() {
        depth_first_search(&mut ones, square);
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(8108, get_used_square_count("flqrgnkx"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1242, get_region_count("flqrgnkx"));
    }
}
