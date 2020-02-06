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

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let combinations = get_combinations_for_capacity(&input, 150);
    println!(
        "how many different combinations of containers can exactly fit all 150 liters of eggnog? {}",
        combinations.len(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"20
15
10
5
5"#;

        assert_eq!(4, get_combinations(&input, 25))
    }
}
