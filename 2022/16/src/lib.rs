use std::collections::{HashMap, HashSet, VecDeque};

use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    CouldNotFindValveError,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: usize,
}

peg::parser! {
    pub grammar scanner() for str {

        rule _() = [' ' | '\n']*

        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule name() -> &'input str
            = l:$(['A'..='Z']*<2>)
                { l }

        rule valve() -> (Valve<'input>, Vec<&'input str>)
            = "Valve"
              _
              name:name()
              _
              "has flow rate="
              flow_rate:number()
              "; tunnel" "s"?
              " lead" "s"?
              " to valve" "s"?
              _
              tunnels:name() ++ ", "
                {
                    (
                        Valve {
                            name,
                            flow_rate,
                        },
                        tunnels,
                    )
                }

        pub rule valves() -> HashMap<Valve<'input>, Vec<Valve<'input>>>
            = _ valves:valve() ++ _
                {

                    let values = valves.clone()
                        .into_iter()
                        .map(|(_, tunnels)| {
                            tunnels
                                .into_iter()
                                .map(|tunnel| valves.iter().find(|valve| valve.0.name == tunnel).unwrap().0)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    let keys = valves.into_iter().map(|(valve, _)| valve);

                    keys
                        .into_iter()
                        .zip(values)
                        .collect()
                }
    }
}

fn get_distance(
    start: &Valve,
    end: &Valve,
    valves: &HashMap<Valve, Vec<Valve>>,
) -> Result<usize, AdventOfCodeError> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((start, 0));
    seen.insert(start);

    while let Some((valve, depth)) = queue.pop_front() {
        if valve == end {
            return Ok(depth);
        }

        seen.insert(valve);

        for tunnel in valves
            .get(valve)
            .ok_or(AdventOfCodeError::CouldNotFindValveError)?
        {
            if !seen.contains(tunnel) {
                queue.push_back((tunnel, depth + 1));
            }
        }
    }

    Err(AdventOfCodeError::CouldNotFindValveError)
}

struct State {
    visited: u16,
    position: usize,
    countdown: usize,
    flow: usize,
}

impl State {
    fn new(visited: u16, position: usize, countdown: usize, flow: usize) -> Self {
        Self {
            visited,
            position,
            countdown,
            flow,
        }
    }
}

fn dfs(visited: u16, countdown: usize, valves: &[&Valve], distances: &[Vec<usize>]) -> usize {
    let mut result = 0;
    let mut stack = VecDeque::new();

    stack.push_front(State::new(visited, 0, countdown, 0));

    while let Some(state) = stack.pop_front() {
        for (next, distance) in distances[state.position].iter().enumerate() {
            if state.visited & (1 << next) == 0 {
                let new_countdown = state.countdown.saturating_sub(1).saturating_sub(*distance);
                if new_countdown > 0 {
                    stack.push_front(State::new(
                        state.visited | 1 << next,
                        next,
                        new_countdown,
                        state.flow + valves[next].flow_rate * new_countdown,
                    ));
                }
                result = result.max(state.flow);
            }
        }
    }

    result
}

pub fn get_part_one(input: &str) -> usize {
    let valves = scanner::valves(input.trim()).unwrap();

    // the only valves in which we're interested…
    let mut flowing_valves = valves
        .keys()
        .filter(|valve| valve.name == "AA" || valve.flow_rate > 0)
        .collect::<Vec<_>>();
    flowing_valves.sort_unstable_by_key(|valve| valve.name);

    // distances from each valve to the others…
    let mut distances = vec![vec![0; flowing_valves.len()]; flowing_valves.len()];
    for i in 0..flowing_valves.len() {
        for j in 0..flowing_valves.len() {
            distances[i][j] = get_distance(flowing_valves[i], flowing_valves[j], &valves).unwrap();
        }
    }

    dfs(0, 30, &flowing_valves, &distances)
}

pub fn get_part_two(input: &str) -> usize {
    let valves = scanner::valves(input.trim()).unwrap();

    // the only valves in which we're interested…
    let mut flowing_valves = valves
        .keys()
        .filter(|valve| valve.name == "AA" || valve.flow_rate > 0)
        .collect::<Vec<_>>();
    flowing_valves.sort_unstable_by_key(|valve| valve.name);

    // distances from each valve to the others…
    let mut distances = vec![vec![0; flowing_valves.len()]; flowing_valves.len()];
    for i in 0..flowing_valves.len() {
        for j in 0..flowing_valves.len() {
            distances[i][j] = get_distance(flowing_valves[i], flowing_valves[j], &valves).unwrap();
        }
    }

    // player visits one subset, elephant visits another;
    // simulate the subsets, find the best value for each pair.
    let all_ones = 2_u16.pow(valves.len() as u32) - 1;
    (0..all_ones)
        .into_par_iter()
        .step_by(2)
        .filter(|visited| visited.count_ones() <= 6)
        .map(|visited| {
            dfs(visited, 26, &flowing_valves, &distances)
                + dfs(!visited ^ 1, 26, &flowing_valves, &distances)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(1_651, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1_707, get_part_two(INPUT));
    }
}
