use std::collections::HashMap;

fn get_rules(input: &str) -> HashMap<(char, char), char> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (pair, insert) = line.split_once(" -> ").unwrap();
            let mut chars = pair.chars();
            (
                (chars.next().unwrap(), chars.next().unwrap()),
                insert.chars().next().unwrap(),
            )
        })
        .collect()
}

/// Get a lookup of element-pair frequencies.
///
/// So the example of `NNCB` becomes:
///
/// NN -> 1
/// NC -> 1
/// CB -> 1
fn get_pair_counts(polymer: &str) -> HashMap<(char, char), usize> {
    polymer
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .fold(HashMap::new(), |mut counts, pair| {
            *counts.entry((pair[0], pair[1])).or_insert(0) += 1;

            counts
        })
}

/// Get a lookup of element frequency.
///
/// So the example of `NNCB` becomes:
///
/// N -> 2
/// C -> 1
/// B -> 1
fn get_element_counts(polymer: &str) -> HashMap<char, usize> {
    polymer.chars().fold(HashMap::new(), |mut counts, element| {
        *counts.entry(element).or_insert(0) += 1;

        counts
    })
}

/// Determine the the difference in frequency between the most- and
/// least-common elements.
///
/// Building the resulting polymer will be impractical (*"This polymer
/// grows quickly."*). Rather than iterate over the elements in the
/// polymer and insert new elements, we can iterate over the *rules*
/// and track the resulting counts.
///
/// For instance, given a polymer of `NN`, this has a pair-count of:
///
/// NN -> 1
///
/// And given a rule of:
///
/// NN -> C
///
/// This results in new pair-counts of:
///
/// NC -> 1
/// CN -> 1
///
/// Similarly, we need to keep track of the frequency of each element.
/// To use the above, example, the frequency of `N` remains unchanged in
/// this step but the frequency of `C` increases equal to the frequency
/// of `NN`.
fn get_most_least_difference_after(input: &str, steps: usize) -> usize {
    let (polymer, rules) = input.trim().split_once("\n\n").unwrap();
    let rules = get_rules(rules);

    let mut pair_counts = get_pair_counts(polymer);
    let mut element_counts = get_element_counts(polymer);

    for _ in 0..steps {
        pair_counts = rules
            .iter()
            .flat_map(|(&(left, right), &insert)| {
                let previous_pair_count = *pair_counts.entry((left, right)).or_insert(0);
                *element_counts.entry(insert).or_insert(0) += previous_pair_count;

                [
                    ((left, insert), previous_pair_count),
                    ((insert, right), previous_pair_count),
                ]
            })
            .fold(HashMap::new(), |mut pair_counts, (pair, count)| {
                *pair_counts.entry(pair).or_insert(0) += count;

                pair_counts
            });
    }

    let mut counts = element_counts
        .iter()
        .map(|(_, &count)| count)
        .collect::<Vec<_>>();
    counts.sort_unstable();

    counts.iter().last().unwrap() - counts.get(0).unwrap()
}
pub fn get_part_one(input: &str) -> usize {
    get_most_least_difference_after(input, 10)
}

pub fn get_part_two(input: &str) -> usize {
    get_most_least_difference_after(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(1588, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2188189693529, get_part_two(INPUT));
    }
}
