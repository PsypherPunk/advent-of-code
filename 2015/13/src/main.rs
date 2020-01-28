use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Seating {
    guest: String,
    neighbour: String,
    happiness: isize,
}

fn get_seating(input: &str) -> HashMap<String, HashMap<String, isize>> {
    let plan =
        Regex::new(r#"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)."#)
            .unwrap();

    let mut seating = HashMap::new();

    input.trim().lines().for_each(|line| {
        let captures = plan.captures(line.trim()).unwrap();
        let mut happiness = captures[3].parse::<isize>().unwrap();
        if &captures[2] == "lose" {
            happiness = -happiness;
        }

        let neighbours = seating
            .entry(captures[1].to_string())
            .or_insert_with(HashMap::new);
        neighbours.insert(captures[4].to_string(), happiness);
    });

    seating
}

/// Calculate the highest-scoring seating plan.
///
/// Remember that `happiness` is bi-directional and circular.
fn get_optimal_seating_plan(input: &str) -> isize {
    let seating = get_seating(&input);

    seating
        .keys()
        .permutations(seating.keys().len())
        .map(|plan| {
            plan.iter()
                .tuple_windows()
                .map(|(&guest, &neighbour)| {
                    *seating.get(guest).unwrap().get(neighbour).unwrap()
                        + *seating.get(neighbour).unwrap().get(guest).unwrap()
                })
                .sum::<isize>()
                + *seating
                    .get(plan[0])
                    .unwrap()
                    .get(plan[plan.len() - 1])
                    .unwrap()
                + *seating
                    .get(plan[plan.len() - 1])
                    .unwrap()
                    .get(plan[0])
                    .unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the total change in happiness for the optimal seating arrangement of the actual guest list? {}",
        get_optimal_seating_plan(&input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seating() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

        assert_eq!(330, get_optimal_seating_plan(&input));
    }
}
