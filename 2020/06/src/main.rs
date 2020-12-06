use std::collections::HashSet;
use std::fs;

fn get_sum_of_yes_counts(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .trim()
                .lines()
                .flat_map(|line| line.chars().collect::<Vec<_>>())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of those counts? {}",
        get_sum_of_yes_counts(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        assert_eq!(11, get_sum_of_yes_counts(&input));
    }
}
