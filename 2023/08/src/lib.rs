//! The input has a very specific structure: each node ending in `A`
//! has a corresponding node ending in `Z`, forming a cycle.
//! 
//! The period of this cycle reaching the node ending in `Z` is the
//! least-common-multiple of the length of both directions with the
//! length of the cycle.
//!
//! A breadth-first-search from each start node finds the length of
//! each cycle. The answer is the total length of the directions.
use std::collections::{HashMap, HashSet, VecDeque};

/// Party one _should_ be a unique instance of the BFS for the nodes
/// named `AAA` and `ZZZ`. However, this doesn't hold true for one of
/// the test cases and has been left as the initial, naive approach.
pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut lines = input.trim().lines();

    let instructions = lines.next().ok_or(format!("bad input: {}", input))?;
    let nodes = lines
        .skip(1)
        .map(|line| {
            let start = &line[..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (start, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let instructions = instructions.chars().cycle();

    let steps = instructions
        .scan("AAA", |node, instruction| match *node {
            "ZZZ" => None,
            current => match nodes.get(current) {
                None => unreachable!(),
                Some((left, right)) => match instruction {
                    'L' => {
                        *node = left;
                        Some(left)
                    }
                    'R' => {
                        *node = right;
                        Some(right)
                    }
                    _ => unreachable!(),
                },
            },
        })
        .count();

    Ok(steps)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut lines = input.trim().lines();

    let mut steps = lines.next().ok_or(format!("bad input: {}", input))?.len();
    let nodes = lines
        .skip(2)
        .map(|line| {
            let start = &line[..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (start, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .for_each(|start| {
            queue.push_back((start, 0));
            seen.insert(start);

            while let Some((node, cost)) = queue.pop_front() {
                if node.ends_with('Z') {
                    steps = num_integer::lcm(steps, cost);
                    break;
                }

                if let Some((left, right)) = nodes.get(node) {
                    if seen.insert(left) {
                        queue.push_back((left, cost + 1));
                    }
                    if seen.insert(right) {
                        queue.push_back((right, cost + 1));
                    }
                }
            }

            queue.clear();
            seen.clear();
        });

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const TWO: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const THREE: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(2), get_part_one(ONE));
        assert_eq!(Ok(6), get_part_one(TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(6), get_part_two(THREE));
    }
}
