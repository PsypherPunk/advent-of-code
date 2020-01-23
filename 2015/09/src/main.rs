use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

use itertools::Itertools;

#[derive(Debug)]
struct Route {
    origin: String,
    destination: String,
    distance: usize,
}

fn get_locations(routes: &[Route]) -> HashSet<String> {
    let distinct = routes
        .iter()
        .flat_map(|route| vec![route.origin.clone(), route.destination.clone()])
        .collect::<Vec<String>>();

    HashSet::from_iter(distinct)
}

fn get_distances(routes: &[Route]) -> HashMap<String, HashMap<String, usize>> {
    let mut distances = HashMap::new();
    for route in routes.iter() {
        let destinations = distances
            .entry(route.origin.clone())
            .or_insert_with(HashMap::new);
        destinations.insert(route.destination.clone(), route.distance);

        let destinations = distances
            .entry(route.destination.clone())
            .or_insert_with(HashMap::new);
        destinations.insert(route.origin.clone(), route.distance);
    }

    distances
}

fn get_routes(input: &str) -> Vec<Route> {
    input
        .trim()
        .lines()
        .map(|line| {
            let pair = line.trim().split(" to ").collect::<Vec<&str>>();
            let origin = String::from(*pair.first().unwrap());
            let location = pair.last().unwrap().split(" = ").collect::<Vec<&str>>();
            let destination = String::from(*location.first().unwrap());
            let distance = location.last().unwrap().parse::<usize>().unwrap();
            Route {
                origin,
                destination,
                distance,
            }
        })
        .collect::<Vec<Route>>()
}

fn get_shortest_route(input: &str) -> usize {
    let routes = get_routes(&input);
    let distances = get_distances(&routes);
    let locations = get_locations(&routes);

    locations
        .iter()
        .permutations(locations.len())
        .map(|permutation| {
            permutation
                .iter()
                .tuple_windows()
                .map(|(&origin, &destination)| {
                    *distances.get(origin).unwrap().get(destination).unwrap()
                })
                .sum::<usize>()
        })
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the distance of the shortest route? {}",
        get_shortest_route(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoded_difference() {
        let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

        assert_eq!(605, get_shortest_route(&input));
    }
}
