use std::collections::HashSet;
use std::fs;

fn get_sum_of_yes_counts_for_anyone(input: &str) -> usize {
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

fn get_char_hashset() -> HashSet<char> {
    ('a'..='z').collect()
}

fn get_sum_of_yes_counts_for_everyone(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .trim()
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(get_char_hashset(), |a, b| {
                    a.intersection(&b).cloned().collect()
                })
                .len()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of those counts? {}",
        get_sum_of_yes_counts_for_anyone(&input),
    );

    println!(
        "What is the sum of those counts? {}",
        get_sum_of_yes_counts_for_everyone(&input),
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

        assert_eq!(11, get_sum_of_yes_counts_for_anyone(&input));
    }

    #[test]
    fn test_part_two() {
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

        assert_eq!(6, get_sum_of_yes_counts_for_everyone(&input));
    }
}
