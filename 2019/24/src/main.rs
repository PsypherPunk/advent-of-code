use std::collections::{HashMap, HashSet};
use std::fs;

type Point = (isize, isize);
type RecursivePoint = (isize, isize, isize);
type Scan = HashMap<Point, char>;

/// Derive a checksum for an Eris scan.
///
/// Treats the grid as a series of 0s or 1s (for empty-space and bugs
/// respectively), turning the result into a binary string and deriving
/// an integer.
fn get_checksum(scan: &Scan) -> isize {
    let binary = (0..5)
        .flat_map(|y| (0..5).map(move |x| (x, y)))
        .map(|tile| match scan.get(&tile).unwrap() {
            '.' => "0",
            '#' => "1",
            _ => panic!("Invalid character at {:?}", tile),
        })
        .collect::<String>();
    isize::from_str_radix(&binary, 2).unwrap()
}

fn get_next_minute(scan: &Scan) -> Scan {
    (0..5)
        .flat_map(|y| {
            (0..5).map(move |x| {
                let adjacent_bugs = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                    .iter()
                    .filter(|&adj| scan.contains_key(adj))
                    .map(|adj| scan.get(adj).unwrap())
                    .filter(|&t| *t == '#')
                    .count();
                (
                    (x, y),
                    match scan.get(&(x, y)).unwrap() {
                        '#' => {
                            if adjacent_bugs == 1 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        '.' => {
                            if adjacent_bugs == 1 || adjacent_bugs == 2 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        _ => panic!("Invalid character at {}, {}", x, y),
                    },
                )
            })
        })
        .collect()
}

fn get_first_duplicate(mut scan: Scan) -> Scan {
    let mut seen = HashSet::new();
    seen.insert(get_checksum(&scan));

    loop {
        let next_minute = get_next_minute(&scan);
        let checksum = get_checksum(&next_minute);
        if seen.contains(&checksum) {
            return next_minute;
        }
        scan = next_minute;
        seen.insert(checksum);
    }
}

fn get_biodiversity_rating(scan: Scan) -> isize {
    (0..5)
        .flat_map(|x| (0..5).map(move |y| (x, y)))
        .map(|(x, y)| match scan.get(&(x, y)).unwrap() {
            '.' => 0,
            '#' => 2_isize.pow((y as u32 * 5) + x as u32),
            _ => panic!("Invalid character at {}, {}", x, y),
        })
        .sum()
}

fn get_scan(input: &str) -> Scan {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect()
}

/// For all known bugs, count the number of bugs in adjacent tiles.
///
/// For bugs on the outer edges (i.e. x or y is 0 or 4), include the
/// containing level (i.e. z - 1).
///
/// For bugs surrounding the inner tile (where x and y are 2), include
/// the contained level (i.e. z + 1).
///
/// For the remaining bugs, include immediate neighbours within the
/// bounds of each scan (i.e. no negative indexes and the inner tile).
fn get_adjacent_bug_count(bugs: &HashSet<RecursivePoint>) -> HashMap<RecursivePoint, usize> {
    let mut count = bugs
        .iter()
        .map(|&tile| (tile, 0))
        .collect::<HashMap<RecursivePoint, usize>>();

    for &(x, y, z) in bugs {
        match x {
            0 => *count.entry((2, 1, z - 1)).or_insert(0) += 1,
            4 => *count.entry((2, 3, z - 1)).or_insert(0) += 1,
            _ => {}
        }
        match y {
            0 => *count.entry((1, 2, z - 1)).or_insert(0) += 1,
            4 => *count.entry((3, 2, z - 1)).or_insert(0) += 1,
            _ => {}
        }
        match (x, y) {
            (1, 2) => {
                for x in 0..=4 {
                    *count.entry((x, 0, z + 1)).or_insert(0) += 1;
                }
            }
            (2, 1) => {
                for y in 0..=4 {
                    *count.entry((0, y, z + 1)).or_insert(0) += 1;
                }
            }
            (2, 3) => {
                for y in 0..=4 {
                    *count.entry((4, y, z + 1)).or_insert(0) += 1;
                }
            }
            (3, 2) => {
                for x in 0..=4 {
                    *count.entry((x, 4, z + 1)).or_insert(0) += 1;
                }
            }
            _ => {}
        }
        if x != 0 && ((x, y) != (3, 2)) {
            *count.entry((x - 1, y, z)).or_insert(0) += 1;
        }
        if x != 4 && ((x, y) != (1, 2)) {
            *count.entry((x + 1, y, z)).or_insert(0) += 1;
        }
        if y != 0 && ((x, y) != (2, 3)) {
            *count.entry((x, y - 1, z)).or_insert(0) += 1;
        }
        if y != 4 && ((x, y) != (2, 1)) {
            *count.entry((x, y + 1, z)).or_insert(0) += 1;
        }
    }
    count
}

fn get_recursive_scan(input: &str) -> HashSet<RecursivePoint> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, tile)| tile == '#')
                .map(move |(x, _)| (x as isize, y as isize, 0_isize))
        })
        .collect()
}

fn get_bug_count_after_minutes(minutes: usize, mut bugs: HashSet<RecursivePoint>) -> usize {
    for _ in 0..minutes {
        for (&tile, &count) in &get_adjacent_bug_count(&bugs) {
            if bugs.contains(&tile) {
                if count != 1 {
                    bugs.remove(&tile);
                }
            } else if count == 1 || count == 2 {
                bugs.insert(tile);
            }
        }
    }

    bugs.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let scan = get_scan(&input);
    let first_duplicate = get_first_duplicate(scan);
    println!(
        "What is the biodiversity rating for the first layout that appears twice? {}",
        get_biodiversity_rating(first_duplicate),
    );

    let recursive_scan = get_recursive_scan(&input);
    println!(
        "…how many bugs are present after 200 minutes? {}",
        get_bug_count_after_minutes(200, recursive_scan),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_string(scan: &Scan) -> String {
        (0..5)
            .map(|y| {
                (0..5)
                    .map(move |x| scan.get(&(x, y)).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn test_one_minute() {
        let input = r#"....#
#..#.
#..##
..#..
#...."#;
        let scan = get_scan(&input);
        let next_minute = get_next_minute(&scan);
        assert_eq!(
            get_string(&next_minute),
            r#"#..#.
####.
###.#
##.##
.##.."#
        );
    }

    #[test]
    fn test_two_minutes() {
        let input = r#"#..#.
####.
###.#
##.##
.##.."#;
        let scan = get_scan(&input);
        let next_minute = get_next_minute(&scan);
        assert_eq!(
            get_string(&next_minute),
            r#"#####
....#
....#
...#.
#.###"#
        );
    }

    #[test]
    fn test_three_minutes() {
        let input = r#"#####
....#
....#
...#.
#.###"#;
        let scan = get_scan(&input);
        let next_minute = get_next_minute(&scan);
        assert_eq!(
            get_string(&next_minute),
            r#"#....
####.
...##
#.##.
.##.#"#
        );
    }

    #[test]
    fn test_four_minutes() {
        let input = r#"#....
####.
...##
#.##.
.##.#"#;
        let scan = get_scan(&input);
        let next_minute = get_next_minute(&scan);
        assert_eq!(
            get_string(&next_minute),
            r#"####.
....#
##..#
.....
##..."#
        );
    }

    #[test]
    fn test_biodiversity_rating() {
        let input = r#".....
.....
.....
#....
.#...
"#;
        let scan = get_scan(&input);
        assert_eq!(2129920, get_biodiversity_rating(scan));
    }

    #[test]
    fn test_recursive_grid() {
        let input = r#"....#
#..#.
#..##
..#..
#...."#;

        let scan = get_recursive_scan(&input);

        assert_eq!(99, get_bug_count_after_minutes(10, scan));
    }
}
