use std::collections::{HashMap, HashSet};
use std::fs;

type Point = (isize, isize);
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

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let scan = get_scan(&input);
    let first_duplicate = get_first_duplicate(scan);
    println!(
        "What is the biodiversity rating for the first layout that appears twice? {}",
        get_biodiversity_rating(first_duplicate),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
