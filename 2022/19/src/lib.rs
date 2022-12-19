use std::collections::{HashSet, VecDeque};

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidBlueprintError(ParseError<LineCol>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cost {
    ore: usize,
    clay: Option<usize>,
    obsidian: Option<usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Blueprint {
    number: usize,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct RobotFactory {
    blueprint: Blueprint,
    ore: usize,
    ore_robots: usize,
    clay: usize,
    clay_robots: usize,
    obsidian: usize,
    obsidian_robots: usize,
    geode: usize,
    geode_robots: usize,
}

impl RobotFactory {
    fn mine(factory: Self) -> Self {
        Self {
            ore: factory.ore + factory.ore_robots,
            clay: factory.clay + factory.clay_robots,
            obsidian: factory.obsidian + factory.obsidian_robots,
            geode: factory.geode + factory.geode_robots,
            ..factory
        }
    }

    fn can_build_geode_robot(&self) -> bool {
        self.ore >= self.blueprint.geode.ore
            && self.obsidian >= self.blueprint.geode.obsidian.unwrap()
    }

    fn build_geode_robot(&mut self) {
        self.ore -= self.blueprint.geode.ore;
        self.obsidian -= self.blueprint.geode.obsidian.unwrap();

        self.geode_robots += 1;
    }

    fn can_build_obsidian_robot(&self) -> bool {
        self.obsidian_robots < self.blueprint.geode.obsidian.unwrap()
            && self.ore >= self.blueprint.obsidian.ore
            && self.clay >= self.blueprint.obsidian.clay.unwrap()
    }

    fn build_obsidian_robot(&mut self) {
        self.ore -= self.blueprint.obsidian.ore;
        self.clay -= self.blueprint.obsidian.clay.unwrap();

        self.obsidian_robots += 1;
    }

    fn can_build_ore_robot(&self) -> bool {
        let max_ore = [
            self.blueprint.ore.ore,
            self.blueprint.clay.ore,
            self.blueprint.obsidian.ore,
            self.blueprint.geode.ore,
        ]
        .into_iter()
        .max()
        .unwrap();

        self.ore_robots < max_ore && self.ore >= self.blueprint.ore.ore
    }

    fn build_ore_robot(&mut self) {
        self.ore -= self.blueprint.ore.ore;

        self.ore_robots += 1;
    }

    fn can_build_clay_robot(&self) -> bool {
        self.clay_robots < self.blueprint.obsidian.clay.unwrap()
            && self.ore >= self.blueprint.clay.ore
    }

    fn build_clay_robot(&mut self) {
        self.ore -= self.blueprint.clay.ore;

        self.clay_robots += 1;
    }
}

peg::parser! {
    pub grammar robot_factory() for str {
        rule _() = [' ' | '\n']*

        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule clay() -> usize
            = _ "and" _ clay:number() _ "clay"
                { clay }

        rule obsidian() -> usize
            = _ "and" _ obsidian:number() _ "obsidian"
                { obsidian }

        rule cost() -> Cost
            = "costs" _ ore:number() _ "ore"
              clay:clay()?
              obsidian:obsidian()?
                {
                    Cost {
                        ore,
                        clay,
                        obsidian,
                    }
                }


        rule blueprint() -> Blueprint
            = "Blueprint" _ number:number() ":" _
              "Each ore robot" _ ore:cost() "." _
              "Each clay robot" _ clay:cost() "." _
              "Each obsidian robot" _ obsidian:cost() "." _
              "Each geode robot" _ geode:cost() "."
                {
                    Blueprint { number, ore, clay, obsidian, geode }
                }

        pub rule blueprints() -> Vec<Blueprint>
            = blueprints:blueprint() ++ _
                { blueprints }
    }
}

impl From<ParseError<LineCol>> for AdventOfCodeError {
    fn from(error: ParseError<LineCol>) -> Self {
        AdventOfCodeError::InvalidBlueprintError(error)
    }
}

fn bfs(factory: RobotFactory) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((factory, 24));

    let mut geodes = 0;

    while let Some((factory, time)) = queue.pop_front() {
        if time == 0 {
            geodes = geodes.max(factory.geode);
            continue;
        }

        if seen.contains(&factory) {
            continue;
        }
        seen.insert(factory);

        if factory.can_build_geode_robot() {
            let mut factory = RobotFactory::mine(factory);
            factory.build_geode_robot();

            queue.push_back((factory, time - 1));
            continue;
        }

        if factory.can_build_obsidian_robot() {
            let mut factory = RobotFactory::mine(factory);
            factory.build_obsidian_robot();

            queue.push_back((factory, time - 1));
            continue;
        }

        if factory.can_build_ore_robot() {
            let mut factory = RobotFactory::mine(factory);
            factory.build_ore_robot();

            queue.push_back((factory, time - 1));
        }

        if factory.can_build_clay_robot() {
            let mut factory = RobotFactory::mine(factory);
            factory.build_clay_robot();

            queue.push_back((factory, time - 1));
        }

        queue.push_back((RobotFactory::mine(factory), time - 1));
    }

    geodes
}

pub fn get_part_one(input: &str) -> usize {
    let blueprints = robot_factory::blueprints(input.trim()).unwrap();

    blueprints
        .into_iter()
        .map(|blueprint| RobotFactory {
            blueprint,
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode: 0,
            geode_robots: 0,
        })
        .map(|factory| factory.blueprint.number * bfs(factory))
        .sum()
}

pub fn get_part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(33, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
