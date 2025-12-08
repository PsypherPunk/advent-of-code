use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Sub;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Default)]
struct DisjointSetUnion {
    parents: HashMap<JunctionBox, JunctionBox>,
}

impl Sub<&JunctionBox> for JunctionBox {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl JunctionBox {
    fn length_squared(&self) -> isize {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn distance_squared(self, rhs: &Self) -> isize {
        (self - rhs).length_squared()
    }
}

impl FromStr for JunctionBox {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(',').collect();

        if parts.len() != 3 {
            return Err(format!("invalid input: {}", s));
        }

        let x = parts[0].parse::<isize>().map_err(|e| e.to_string())?;
        let y = parts[1].parse::<isize>().map_err(|e| e.to_string())?;
        let z = parts[2].parse::<isize>().map_err(|e| e.to_string())?;

        Ok(Self { x, y, z })
    }
}

impl DisjointSetUnion {
    fn parent(&self, mut junction_box: JunctionBox) -> JunctionBox {
        while let Some(parent) = self.parents.get(&junction_box) {
            junction_box = *parent;
        }

        junction_box
    }

    fn union(&mut self, a: JunctionBox, b: JunctionBox) -> usize {
        let (a_parent, b_parent) = (self.parent(a), self.parent(b));

        match a_parent.cmp(&b_parent) {
            Ordering::Equal => 0,
            _ => {
                self.parents.insert(a_parent, b_parent);
                1
            }
        }
    }
}

pub fn get_part_one(input: &str, pairs: usize) -> Result<usize, String> {
    let junction_boxes = input
        .lines()
        .map(|line| line.parse::<JunctionBox>())
        .collect::<Result<Vec<_>, _>>()?;

    let distances = {
        let mut distances = junction_boxes
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| (a, b, a.distance_squared(b)))
            .collect_vec();

        distances.sort_unstable_by_key(|(_, _, distance)| *distance);
        distances.truncate(pairs);

        distances
    };

    let mut dsu = DisjointSetUnion::default();
    let mut circuits: HashMap<JunctionBox, HashSet<JunctionBox>> = HashMap::new();

    for (a, b, _) in distances {
        dsu.union(*a, *b);
    }

    for junction in junction_boxes {
        circuits
            .entry(dsu.parent(junction))
            .or_default()
            .insert(junction);
    }

    let mut largest = circuits.values().map(HashSet::len).collect_vec();
    largest.sort_unstable();
    largest.reverse();

    Ok(largest[..3].iter().product())
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let junction_boxes = input
        .lines()
        .map(|line| line.parse::<JunctionBox>())
        .collect::<Result<Vec<_>, _>>()?;

    let distances = {
        let mut distances = junction_boxes
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| (a, b, a.distance_squared(b)))
            .collect_vec();

        distances.sort_unstable_by_key(|(_, _, distance)| *distance);

        distances
    };

    let mut dsu = DisjointSetUnion::default();
    let mut connections = 0;

    let last_pair = distances.iter().find(|(&a, &b, _)| {
        connections += dsu.union(a, b);

        connections == junction_boxes.len().saturating_sub(1)
    });

    match last_pair {
        Some((a, b, _)) => Ok((a.x * b.x) as usize),
        None => Err("could not find last pair".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(40), get_part_one(INPUT, 10));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(25272), get_part_two(INPUT));
    }
}
