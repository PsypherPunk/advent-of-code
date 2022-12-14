use std::collections::HashSet;

fn get_scan(input: &str) -> HashSet<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coordinate| {
                    let (x, y) = coordinate.split_once(',').unwrap();
                    let x = x.parse::<usize>().unwrap();
                    let y = y.parse::<usize>().unwrap();

                    (x, y)
                })
                .fold(
                    (HashSet::new(), None::<(usize, usize)>),
                    |(mut rocks, last), (x, y)| {
                        match last {
                            None => {
                                rocks.insert((x, y));
                            }
                            Some(rock) => {
                                let structure = (rock.0.min(x)..=rock.0.max(x)).flat_map(|dx| {
                                    (rock.1.min(y)..=rock.1.max(y)).map(move |dy| (dx, dy))
                                });
                                rocks.extend(structure);
                            }
                        };

                        (rocks, Some((x, y)))
                    },
                )
                .0
        })
        .fold(HashSet::new(), |mut acc, coordinates| {
            acc.extend(coordinates);

            acc
        })
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
    0
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
        assert_eq!(2, get_part_two(INPUT));
    }
}
