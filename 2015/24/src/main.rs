use std::fs;

use itertools::Itertools;

fn get_packages(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn get_smallest_groups(packages: &[usize], groups: usize) -> Option<Vec<Vec<usize>>> {
    let compartment_weight = packages.iter().sum::<usize>() / groups;

    for group_size in 1..packages.len() {
        let optimal_groups = packages
            .iter()
            .cloned()
            .combinations(group_size)
            .filter(|package_group| package_group.iter().sum::<usize>() == compartment_weight)
            .collect::<Vec<Vec<usize>>>();
        if !optimal_groups.is_empty() {
            return Some(optimal_groups);
        }
    }
    None
}

fn get_quantum_entanglement(packages: &[usize]) -> usize {
    packages.iter().product()
}

fn get_passenger_compartment(input: &str, groups: usize) -> Vec<usize> {
    let packages = get_packages(&input);
    let mut smallest_groups = get_smallest_groups(&packages, groups).unwrap();

    if smallest_groups.len() > 1 {
        smallest_groups
            .sort_by(|a, b| get_quantum_entanglement(a).cmp(&get_quantum_entanglement(b)));
        smallest_groups.first().unwrap().to_vec()
    } else {
        smallest_groups.get(0).unwrap().to_vec()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let passenger_packages = get_passenger_compartment(&input, 3);
    println!(
        "What is the quantum entanglement of the first group of packages in the ideal configuration? {}",
        get_quantum_entanglement(&passenger_packages),
    );

    let passenger_packages = get_passenger_compartment(&input, 4);
    println!(
        "Now, what is the quantum entanglement of the first group of packages in the ideal configuration? {}",
        get_quantum_entanglement(&passenger_packages),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"1
2
3
4
5
7
8
9
10
11";
        let packages = get_packages(&input);

        let smallest_groups = get_smallest_groups(&packages, 3).unwrap();

        assert_eq!(1, smallest_groups.len());
        assert_eq!(99, get_quantum_entanglement(&smallest_groups[0]));
    }

    #[test]
    fn test_part2() {
        let input = r"1
2
3
4
5
7
8
9
10
11";

        let passenger_packages = get_passenger_compartment(&input, 4);

        assert_eq!(44, get_quantum_entanglement(&passenger_packages));
    }
}
