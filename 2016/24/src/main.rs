use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

type Point = (usize, usize);

#[derive(Debug)]
enum Location {
    OpenPassage,
    Wall,
    Target(u32),
}

struct Hvac {
    locations: HashMap<Point, Location>,
}

impl Hvac {
    fn get_targets(&self) -> Vec<&Point> {
        let mut targets = self
            .locations
            .iter()
            .filter(|(_, location)| matches!(location, Location::Target(_)))
            .collect::<Vec<_>>();

        targets.sort_by(|(_, a), (_, b)| {
            let a = match a {
                Location::Target(a) => a,
                _ => panic!(),
            };
            let b = match b {
                Location::Target(b) => b,
                _ => panic!(),
            };
            a.cmp(b)
        });

        targets.iter().map(|(point, _)| *point).collect()
    }

    fn get_neighbours(&self, (x, y): &Point) -> Vec<Point> {
        [(*x, y - 1), (*x, y + 1), (*x - 1, *y), (*x + 1, *y)]
            .iter()
            .filter(|&neighbour| !matches!(self.locations.get(neighbour).unwrap(), Location::Wall))
            .cloned()
            .collect()
    }

    fn bfs(&self, a: &Point, b: &Point) -> usize {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back((*a, 0));
        while !queue.is_empty() {
            let (current, distance) = queue.pop_front().unwrap();
            if current == *b {
                return distance;
            }
            seen.insert(*a);

            let neighbours = self
                .get_neighbours(&current)
                .into_iter()
                .filter(|neighbour| !seen.contains(neighbour))
                .collect::<Vec<_>>();

            neighbours.iter().for_each(|&neighbour| {
                seen.insert(neighbour);
                queue.push_back((neighbour, distance + 1));
            });
        }

        panic!("Could not find path from {:?} to {:?}.", a, b);
    }

    fn get_distances_between_targets(&self, targets: &[&Point]) -> HashMap<Point, usize> {
        targets
            .iter()
            .enumerate()
            .flat_map(|(from, a)| {
                targets
                    .iter()
                    .enumerate()
                    .filter(|(to, _)| *to != from)
                    .map(|(to, b)| ((from, to), (*a, *b)))
                    .collect::<Vec<_>>()
            })
            .map(|((from, to), (a, b))| ((from, to), self.bfs(a, b)))
            .collect()
    }

    fn get_shortest_path(&self) -> usize {
        let targets = self.get_targets();
        let distances = self.get_distances_between_targets(&targets);

        (0..targets.len())
            .permutations(targets.len())
            .filter(|permutation| *permutation.first().unwrap() == 0)
            .map(|permutation| {
                permutation
                    .iter()
                    .tuple_windows::<(&usize, &usize)>()
                    .map(|(from, to)| distances.get(&(*from, *to)).unwrap())
                    .sum()
            })
            .min()
            .unwrap()
    }

    fn get_shortest_path_with_return(&self) -> usize {
        let targets = self.get_targets();
        let distances = self.get_distances_between_targets(&targets);

        let mut steps = (0..targets.len()).collect::<Vec<_>>();
        steps.insert(0, 0);
        steps
            .iter()
            .permutations(steps.len())
            .filter(|permutation| {
                *permutation.first().unwrap() == &0 && *permutation.last().unwrap() == &0
            })
            .map(|permutation| {
                permutation
                    .into_iter()
                    .tuple_windows::<(&usize, &usize)>()
                    .map(|(from, to)| distances.get(&(*from, *to)).unwrap())
                    .sum()
            })
            .min()
            .unwrap()
    }
}

impl FromStr for Hvac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, char)| {
                    let location = match char {
                        '.' => Location::OpenPassage,
                        '#' => Location::Wall,
                        t if t.is_numeric() => Location::Target(t.to_digit(10).unwrap()),
                        _ => panic!("Invalid location: {}", char),
                    };
                    ((x, y), location)
                })
            })
            .collect::<HashMap<Point, Location>>();

        Ok(Hvac { locations })
    }
}

impl Display for Hvac {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let (max_x, max_y) = self.locations.keys().max().unwrap();

        let output = (0..=*max_y)
            .map(|y| {
                (0..=*max_x)
                    .map(|x| match self.locations.get(&(x, y)).unwrap() {
                        Location::OpenPassage => ".".to_string(),
                        Location::Wall => "#".to_string(),
                        Location::Target(t) => t.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let hvac = Hvac::from_str(&input).unwrap();

    println!(
        "…what is the fewest number of steps…? {}",
        hvac.get_shortest_path(),
    );

    println!(
        "What is the fewest number of steps…and then return to 0? {}",
        hvac.get_shortest_path_with_return(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"###########
#0.1.....2#
#.#######.#
#4.......3#
###########"#;

        let hvac = Hvac::from_str(&input).unwrap();

        assert_eq!(2, hvac.bfs(&(1, 1), &(1, 3)));
        assert_eq!(4, hvac.bfs(&(1, 3), &(3, 1)));
        assert_eq!(14, hvac.get_shortest_path());
    }
}
