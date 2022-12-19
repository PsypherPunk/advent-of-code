use std::collections::{HashSet, VecDeque};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Blueprint {
    number: usize,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
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
                        clay: clay.unwrap_or(0),
                        obsidian: obsidian.unwrap_or(0),
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

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
struct RobotFactory {
    ore: usize,
    ore_robots: usize,
    clay: usize,
    clay_robots: usize,
    obsidian: usize,
    obsidian_robots: usize,
    geode: usize,
    geode_robots: usize,
    time: usize,
}

impl RobotFactory {
    fn new() -> Self {
        Self {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode: 0,
            geode_robots: 0,
            time: 0,
        }
    }

    fn mine(factory: Self) -> Self {
        Self {
            ore: factory.ore + factory.ore_robots,
            clay: factory.clay + factory.clay_robots,
            obsidian: factory.obsidian + factory.obsidian_robots,
            geode: factory.geode + factory.geode_robots,
            time: factory.time + 1,
            ..factory
        }
    }

    fn can_build_geode_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.geode.ore && self.obsidian >= blueprint.geode.obsidian
    }

    fn build_geode_robot(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.geode.ore;
        self.obsidian -= blueprint.geode.obsidian;

        self.geode_robots += 1;
    }

    fn can_build_ore_robot(&self, blueprint: &Blueprint, max_ore_cost: usize) -> bool {
        self.ore >= blueprint.ore.ore && self.ore_robots < max_ore_cost
    }

    fn build_ore_robot(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.ore.ore;
        self.ore_robots += 1;
    }

    fn can_build_clay_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.clay.ore && self.clay_robots < blueprint.obsidian.clay
    }

    fn build_clay_robot(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.clay.ore;
        self.clay_robots += 1;
    }

    fn can_build_obsidian_robot(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.obsidian.ore
            && self.clay >= blueprint.obsidian.clay
            && self.obsidian_robots < blueprint.geode.obsidian
    }

    fn build_obsidian_robot(&mut self, blueprint: &Blueprint) {
        self.ore -= blueprint.obsidian.ore;
        self.clay -= blueprint.obsidian.clay;
        self.obsidian_robots += 1;
    }
}

fn bfs(blueprint: Blueprint, time: usize) -> usize {
    let mut queue = VecDeque::new();
    let factory = RobotFactory::new();
    let mut geodes = 0;

    queue.push_back(factory);

    let mut seen = HashSet::new();

    let max_ore_cost = blueprint
        .clay
        .ore
        .max(blueprint.obsidian.ore)
        .max(blueprint.geode.ore);

    while let Some(factory) = queue.pop_front() {
        geodes = geodes.max(factory.geode);

        if factory.geode < geodes.saturating_sub(1) {
            continue;
        };

        if seen.contains(&factory) {
            continue;
        }

        if factory.time == time {
            continue;
        }

        seen.insert(factory);

        if factory.can_build_geode_robot(&blueprint) {
            let mut factory = RobotFactory::mine(factory);
            factory.build_geode_robot(&blueprint);

            queue.push_back(factory);
            continue;
        }

        if factory.can_build_ore_robot(&blueprint, max_ore_cost) {
            let mut factory = RobotFactory::mine(factory);
            factory.build_ore_robot(&blueprint);

            queue.push_back(factory);
        }

        if factory.can_build_clay_robot(&blueprint) {
            let mut factory = RobotFactory::mine(factory);
            factory.build_clay_robot(&blueprint);

            queue.push_back(factory);
        }

        if factory.can_build_obsidian_robot(&blueprint) {
            let mut factory = RobotFactory::mine(factory);
            factory.build_obsidian_robot(&blueprint);

            queue.push_back(factory);
            continue;
        }

        queue.push_back(RobotFactory::mine(factory));
    }

    geodes
}

pub fn get_part_one(input: &str) -> usize {
    let blueprints = robot_factory::blueprints(input.trim()).unwrap();

    blueprints
        .into_par_iter()
        .map(|blueprint| blueprint.number * bfs(blueprint, 24))
        .sum()
}

pub fn get_part_two(input: &str) -> usize {
    let blueprints = robot_factory::blueprints(input.trim()).unwrap();

    blueprints[..3.min(blueprints.len())]
        .into_par_iter()
        .map(|blueprint| bfs(*blueprint, 32))
        .product()
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

    // not passing but gets the right answer for Part 2…?!
    #[test]
    fn test_part_two() {
        assert_eq!(56 * 62, get_part_two(INPUT));
    }
}
