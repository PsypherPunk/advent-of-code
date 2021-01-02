use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, weight) = match s.trim().split_whitespace().collect::<Vec<_>>()[..] {
            [a, b] => (a.to_string(), b[1..(b.len() - 1)].parse().unwrap()),
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        };

        Ok(Self {
            name,
            weight,
            children: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct Tower {
    programs: HashMap<String, Program>,
}

impl FromStr for Tower {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let programs = s
            .trim()
            .lines()
            .map(|line| match line.split(" -> ").collect::<Vec<_>>()[..] {
                [a, b] => {
                    let mut program = Program::from_str(a).unwrap();
                    program
                        .children
                        .extend(b.split(", ").map(|n| n.to_string()).collect::<Vec<_>>());
                    (program.name.clone(), program)
                }
                [a] => {
                    let program = Program::from_str(a).unwrap();
                    (program.name.clone(), program)
                }
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            })
            .collect();

        Ok(Self { programs })
    }
}

fn get_mode(numbers: Vec<usize>) -> usize {
    let mut counts = HashMap::new();

    numbers
        .iter()
        .copied()
        .max_by_key(|&number| {
            let count = counts.entry(number).or_insert(0);
            *count += 1;
            *count
        })
        .unwrap()
}

impl Tower {
    pub fn get_bottom_program(&self) -> String {
        let parents = self
            .programs
            .values()
            .map(|k| k.name.clone())
            .collect::<HashSet<_>>();
        let children = self
            .programs
            .values()
            .map(|program| program.children.clone())
            .flatten()
            .collect::<HashSet<_>>();

        parents.difference(&children).next().unwrap().clone()
    }

    fn get_total_program_weight(&self, program: &Program) -> usize {
        program.weight
            + program
                .children
                .iter()
                .map(|child| {
                    let child = self.programs.get(child).unwrap();
                    self.get_total_program_weight(child)
                })
                .sum::<usize>()
    }

    fn get_parent(&self, child: &Program) -> &Program {
        self.programs
            .values()
            .find(|parent| parent.children.contains(&child.name))
            .unwrap()
    }

    fn get_mismatch_from_program(&self, name: &str) -> &Program {
        let program = self.programs.get(name).unwrap();

        let child_weights = program
            .children
            .iter()
            .map(|child| {
                let child = self.programs.get(child).unwrap();
                self.get_total_program_weight(child)
            })
            .collect();

        let mode = get_mode(child_weights);

        let unbalanced_child = program.children.iter().find(|&child| {
            let child = self.programs.get(child).unwrap();
            self.get_total_program_weight(child) != mode
        });

        match unbalanced_child {
            Some(child) => self.get_mismatch_from_program(child),
            None => program,
        }
    }

    pub fn get_corrected_wrong_weight(&self) -> usize {
        let bottom = self.get_bottom_program();

        let dodgy_program = self.get_mismatch_from_program(&bottom);
        let parent = self.get_parent(dodgy_program);
        let child_weights = parent
            .children
            .iter()
            .map(|child| {
                let child = self.programs.get(child).unwrap();
                self.get_total_program_weight(child)
            })
            .collect::<Vec<_>>();
        let mode = get_mode(child_weights);
        let difference = self.get_total_program_weight(dodgy_program) - mode;

        dodgy_program.weight - difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"pbga (66)
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

    #[test]
    fn test_part_one() {
        let tower = Tower::from_str(&INPUT).unwrap();

        assert_eq!("tknk".to_string(), tower.get_bottom_program());
    }

    #[test]
    fn test_part_two() {
        let tower = Tower::from_str(&INPUT).unwrap();

        assert_eq!(60, tower.get_corrected_wrong_weight());
    }
}
