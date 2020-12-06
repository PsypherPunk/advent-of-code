use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs;

type Point = (isize, isize);

struct Cluster {
    grid: HashMap<Point, Node>,
}

#[derive(PartialEq)]
struct Node {
    size: usize,
    used: usize,
    avail: usize,
    used_percent: usize,
}

impl Cluster {
    fn from_str(input: &str) -> Self {
        let grid: HashMap<Point, Node> = input
            .trim()
            .lines()
            .skip(2)
            .map(|line| {
                let details = line.trim().split_whitespace().collect::<Vec<_>>();
                let node_details = details[0].split('-').collect::<Vec<_>>();

                let x = node_details[1].replace('x', "").parse::<isize>().unwrap();
                let y = node_details[2].replace('y', "").parse::<isize>().unwrap();

                ((x, y), Node::from_str(&line))
            })
            .collect();

        Self { grid }
    }

    fn get_viable_pairs(&self) -> Vec<(&Node, &Node)> {
        let mut viable_pairs = Vec::new();

        self.grid
            .values()
            .filter(|&node_a| node_a.used > 0)
            .for_each(|node_a| {
                self.grid
                    .values()
                    .filter(|&node_b| node_b != node_a)
                    .for_each(|node_b| {
                        if node_a.used < node_b.avail {
                            viable_pairs.push((node_a, node_b));
                        }
                    })
            });

        viable_pairs
    }
}

impl Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let &(max_x, max_y) = self.grid.keys().max().unwrap();
        let moves = self
            .get_viable_pairs()
            .iter()
            .flat_map(|pair| vec![pair.0, pair.1])
            .collect::<Vec<_>>();

        let output = (0..=max_x)
            .map(|x| {
                (0..=max_y)
                    .map(|y| {
                        let node = self.grid.get(&(x, y)).unwrap();
                        match node {
                            _ if (x, y) == (max_x, 0) => 'G',
                            _ if (x, y) == (0, 0) => 'A',
                            _ if node.used == 0 => '_',
                            _ if !moves.contains(&node) => '#',
                            _ => '.',
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

impl Node {
    fn from_str(input: &str) -> Self {
        let details = input.trim().split_whitespace().collect::<Vec<_>>();

        Self {
            size: details[1].replace('T', "").parse::<usize>().unwrap(),
            used: details[2].replace('T', "").parse::<usize>().unwrap(),
            avail: details[3].replace('T', "").parse::<usize>().unwrap(),
            used_percent: details[4].replace('%', "").parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let cluster = Cluster::from_str(&input);

    println!(
        "How many viable pairs of nodes are there? {}",
        cluster.get_viable_pairs().len(),
    );

    print!("{}", cluster);
}
