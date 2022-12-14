use std::collections::HashSet;

use itertools::Itertools;

pub fn get_scan(input: &str) -> HashSet<(usize, usize)> {
    input
        .trim()
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|coordinate| {
                    let (x, y) = coordinate.split_once(',').unwrap();
                    let x = x.parse::<usize>().unwrap();
                    let y = y.parse::<usize>().unwrap();

                    (x, y)
                })
                .tuple_windows()
                .flat_map(|(start, end)| {
                    (start.0.min(end.0)..=start.0.max(end.0))
                        .flat_map(move |x| {
                            (start.1.min(end.1)..=start.1.max(end.1))
                                .map(move |y| (x, y))
                        })

                })
        })
        .collect()
}

pub fn get_part_one(input: &str) -> usize {
    let mut scan = get_scan(input);

    let void = scan.iter().map(|(_, y)| *y).max().unwrap();
    let mut units = 0;

    'outer: loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 > void {
                break 'outer;
            }

            if !scan.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !scan.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !scan.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                scan.insert(sand);
                break;
            }
        }

        units += 1;
    }

    units
}

pub fn get_part_two(input: &str) -> usize {
    let mut scan = get_scan(input);

    let floor = scan.iter().map(|(_, y)| *y).max().unwrap() + 2;
    let mut units = 0;

    loop {
        if scan.contains(&(500, 0)) {
            break;
        }

        let mut sand = (500, 0);

        loop {
            if sand.1 + 1 == floor {
                scan.insert(sand);
                break;
            } else if !scan.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !scan.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !scan.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                scan.insert(sand);
                break;
            }
        }

        units += 1;
    }

    units
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(24, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(93, get_part_two(INPUT));
    }
}
