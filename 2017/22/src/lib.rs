use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use num_complex::Complex;

struct Virus {
    position: Complex<isize>,
    direction: Complex<isize>,
}

pub struct Cluster {
    nodes: HashMap<Complex<isize>, bool>,
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
                line.chars().enumerate().map(move |(x, char)| {
                    (
                        Complex::new((x as isize) - zero, zero - (y as isize)),
                        char == '#',
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
        let min = self
            .nodes
            .keys()
            .min_by(|a, b| (a.re, a.im).cmp(&(b.re, b.im)))
            .unwrap();
        let max = self
            .nodes
            .keys()
            .max_by(|a, b| (a.re, a.im).cmp(&(b.re, b.im)))
            .unwrap();

        let mut output = String::new();
        let mut y = max.im;

        while y >= min.im {
            let mut x = min.re;
            while x <= max.re {
                let node = Complex::new(x, y);
                if self.virus.position == node {
                    output.push('*');
                    x += 1;
                    continue;
                }
                let ch = match self.nodes.get(&node) {
                    Some(infected) => match *infected {
                        true => '#',
                        false => '.',
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
        let right = Complex::i().powi(1);
        let left = Complex::i().powi(-1);

        let current_is_infected = self.nodes.entry(self.virus.position).or_insert(false);
        match *current_is_infected {
            true => {
                self.virus.direction *= left;
                *current_is_infected = false;
            }
            false => {
                self.virus.direction *= right;
                *current_is_infected = true;
                self.infections += 1;
            }
        };
        self.virus.position += self.virus.direction;
    }

    pub fn bursts_of_activity(&mut self, bursts: usize) {
        for _ in 0..bursts {
            self.burst();
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
}
