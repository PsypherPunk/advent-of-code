use std::fs;

use itertools::Itertools;

fn get_containers(input: &str) -> Vec<u16> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect()
}

fn get_combinations_for_capacity(input: &str, capacity: u16) -> Vec<Vec<u16>> {
    let containers = get_containers(&input);

    (1..containers.len())
        .flat_map(move |count| {
            containers
                .iter()
                .combinations(count)
                .filter(|combination| combination.iter().fold(0, |a, &b| a + *b) == capacity)
                .map(|combination| combination.iter().map(|&c| *c).collect::<Vec<u16>>())
                .collect::<Vec<Vec<u16>>>()
        })
        .collect::<Vec<Vec<u16>>>()
}

fn get_smallest_combinations(combinations: &[Vec<u16>]) -> Vec<Vec<u16>> {
    let lengths = combinations
        .iter()
        .map(|combination| combination.len())
        .collect::<Vec<usize>>();

    let smallest = *lengths.iter().min().unwrap();

    combinations
        .iter()
        .filter(|combination| combination.len() == smallest)
        .map(|combination| combination.iter().copied().collect::<Vec<u16>>())
        .collect::<Vec<Vec<u16>>>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let combinations = get_combinations_for_capacity(&input, 150);

    println!(
        "how many different combinations of containers can exactly fit all 150 liters of eggnog? {}",
        combinations.len(),
    );

    println!(
        "How many different ways can you fill that number of containers…? {}",
        get_smallest_combinations(&combinations).len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"20
15
10
5
5"#;
        let combinations = get_combinations_for_capacity(&input, 25);

        assert_eq!(4, combinations.len());
    }

    #[test]
    fn test_part2() {
        let input = r#"20
15
10
5
5"#;
        let combinations = get_combinations_for_capacity(&input, 25);

        assert_eq!(3, get_smallest_combinations(&combinations).len());
    }
}
