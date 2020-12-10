use itertools::Itertools;
use std::collections::HashMap;

pub fn get_adapter_joltages(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn get_joltage_differences(joltages: &[usize]) -> HashMap<usize, usize> {
    let mut joltages = joltages.to_owned();
    joltages.insert(0, 0);
    joltages.sort_unstable();

    let mut diffs = HashMap::new();
    joltages
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .for_each(|diff| {
            *diffs.entry(diff).or_insert(0) += 1;
        });
    *diffs.entry(3).or_insert(0) += 1;

    diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &'static str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    const LARGE: &'static str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn test_part_one_small() {
        let joltages = get_adapter_joltages(&SMALL);
        let diffs = get_joltage_differences(&joltages);

        assert_eq!(35, diffs.get(&1).unwrap() * diffs.get(&3).unwrap());
    }

    #[test]
    fn test_part_one_large() {
        let joltages = get_adapter_joltages(&LARGE);
        let diffs = get_joltage_differences(&joltages);

        assert_eq!(220, diffs.get(&1).unwrap() * diffs.get(&3).unwrap());
    }
}
