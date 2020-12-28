use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Eq)]
struct Program {
    name: String,
    weight: usize,
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Program {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, weight) = match s.trim().split_whitespace().collect::<Vec<_>>()[..] {
            [a, b] => (a.to_string(), b[1..(b.len() - 1)].parse().unwrap()),
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        };

        Ok(Self { name, weight })
    }
}

#[derive(Debug)]
pub struct Tower {
    programs: HashMap<Program, Vec<String>>,
}

impl FromStr for Tower {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let programs = s
            .trim()
            .lines()
            .map(|line| match line.split(" -> ").collect::<Vec<_>>()[..] {
                [a, b] => (
                    Program::from_str(a).unwrap(),
                    b.split(", ").map(|n| n.to_string()).collect::<Vec<_>>(),
                ),
                [a] => (Program::from_str(a).unwrap(), Vec::new()),
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            })
            .collect();

        Ok(Self { programs })
    }
}

impl Tower {
    pub fn get_bottom_program(&self) -> String {
        let parents = self
            .programs
            .keys()
            .map(|k| k.name.clone())
            .collect::<HashSet<_>>();
        let children = self
            .programs
            .values()
            .flatten()
            .cloned()
            .collect::<HashSet<_>>();

        parents.difference(&children).next().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;

        let tower = Tower::from_str(&input).unwrap();

        assert_eq!("tknk".to_string(), tower.get_bottom_program());
    }
}
