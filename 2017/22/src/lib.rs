use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use num_complex::Complex;

enum Node {
    Clean,
    Weakened,
    Flagged,
    Infected,
}

struct Virus {
    position: Complex<isize>,
    direction: Complex<isize>,
}

pub struct Cluster {
    nodes: HashMap<Complex<isize>, Node>,
    virus: Virus,
    infections: usize,
}

impl FromStr for Cluster {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let zero = (s.trim().lines().count() / 2) as isize;

        let nodes = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, ch)| {
                    (
                        Complex::new((x as isize) - zero, zero - (y as isize)),
                        match ch {
                            '.' => Node::Clean,
                            '#' => Node::Infected,
                            _ => panic!(r#"¯\_(ツ)_/¯"#),
                        },
                    )
                })
            })
            .collect();

        Ok(Self {
            nodes,
            virus: Virus::new(),
            infections: 0,
        })
    }
}

impl Display for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_string())
    }
}

impl Cluster {
    fn as_string(&self) -> String {
        let min_x = self.nodes.keys().map(|node| node.re).min().unwrap();
        let max_x = self.nodes.keys().map(|node| node.re).max().unwrap();
        let min_y = self.nodes.keys().map(|node| node.im).min().unwrap();
        let max_y = self.nodes.keys().map(|node| node.im).max().unwrap();

        let mut output = String::new();
        let mut y = max_y;

        while y >= min_y {
            let mut x = min_x;
            while x <= max_x {
                let node = Complex::new(x, y);
                if self.virus.position == node {
                    output.push('*');
                    x += 1;
                    continue;
                }
                let ch = match self.nodes.get(&node) {
                    Some(infected) => match *infected {
                        Node::Infected => '#',
                        Node::Clean => '.',
                        Node::Weakened => 'W',
                        Node::Flagged => 'F',
                    },
                    None => '.',
                };
                output.push(ch);
                x += 1;
            }
            output.push('\n');
            y -= 1;
        }

        output
    }

    fn burst(&mut self) {
        let left = Complex::i().powi(1);
        let right = Complex::i().powi(-1);

        let current_is_infected = self.nodes.entry(self.virus.position).or_insert(Node::Clean);
        match *current_is_infected {
            Node::Infected => {
                self.virus.direction *= right;
                *current_is_infected = Node::Clean;
            }
            Node::Clean => {
                self.virus.direction *= left;
                *current_is_infected = Node::Infected;
                self.infections += 1;
            }
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        };
        self.virus.position += self.virus.direction;
    }

    pub fn bursts_of_activity(&mut self, bursts: usize) {
        for _ in 0..bursts {
            self.burst();
        }
    }

    fn evolved_burst(&mut self) {
        let left = Complex::i().powi(1);
        let right = Complex::i().powi(-1);
        let reverse = Complex::i().powi(-2);

        let current_is_infected = self.nodes.entry(self.virus.position).or_insert(Node::Clean);
        match *current_is_infected {
            Node::Clean => {
                self.virus.direction *= left;
                *current_is_infected = Node::Weakened;
            }
            Node::Weakened => {
                *current_is_infected = Node::Infected;
                self.infections += 1;
            }
            Node::Infected => {
                self.virus.direction *= right;
                *current_is_infected = Node::Flagged;
            }
            Node::Flagged => {
                *current_is_infected = Node::Clean;
                self.virus.direction *= reverse;
            }
        };
        self.virus.position += self.virus.direction;
    }

    pub fn evolved_bursts_of_activity(&mut self, bursts: usize) {
        for _ in 0..bursts {
            self.evolved_burst();
        }
    }

    pub fn get_infections(&self) -> usize {
        self.infections
    }
}

impl Virus {
    fn new() -> Self {
        Self {
            position: Complex::new(0, 0),
            direction: Complex::new(0, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..#
#..
..."#;

    #[test]
    fn test_part_one_7() {
        let mut cluster = Cluster::from_str(&INPUT).unwrap();

        cluster.bursts_of_activity(7);

        assert_eq!(5, cluster.infections);
    }

    #[test]
    fn test_part_one_70() {
        let mut cluster = Cluster::from_str(&INPUT).unwrap();

        cluster.bursts_of_activity(70);

        assert_eq!(41, cluster.infections);
    }

    #[test]
    fn test_part_one_10000() {
        let mut cluster = Cluster::from_str(&INPUT).unwrap();

        cluster.bursts_of_activity(10_000);

        assert_eq!(5_587, cluster.infections);
    }

    #[test]
    fn test_part_two_100() {
        let mut cluster = Cluster::from_str(&INPUT).unwrap();

        cluster.evolved_bursts_of_activity(100);

        assert_eq!(26, cluster.infections);
    }

    #[test]
    fn test_part_two_10000000() {
        let mut cluster = Cluster::from_str(&INPUT).unwrap();

        cluster.evolved_bursts_of_activity(10_000_000);

        assert_eq!(2_511_944, cluster.infections);
    }
}
