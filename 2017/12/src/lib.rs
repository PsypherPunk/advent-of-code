use std::collections::HashSet;

use pathfinding::undirected::connected_components::separate_components;

fn get_pipes(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (input, outputs) = match line.split("<->").collect::<Vec<_>>()[..] {
                [input, output] => (input, output),
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            };
            let mut outputs = outputs.split(',').map(|num| num.trim()).collect::<Vec<_>>();

            outputs.insert(0, input.trim());

            outputs
        })
        .collect()
}

pub fn get_group_size_for(pipe: &str, input: &str) -> usize {
    let pipes = get_pipes(&input);

    let (indices, _) = separate_components(&pipes);
    let zero = indices.get(pipe).unwrap();

    indices.values().filter(|&group| group == zero).count()
}

pub fn get_group_count(input: &str) -> usize {
    let pipes = get_pipes(&input);

    let (_, groups) = separate_components(&pipes);

    groups.iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#;

        assert_eq!(6, get_group_size_for("0", &input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#;

        assert_eq!(2, get_group_count(input));
    }
}
