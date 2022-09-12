fn get_metadata_sum(nodes: &mut impl Iterator<Item = usize>) -> usize {
    let child_count = nodes.next().unwrap();
    let metadata_count = nodes.next().unwrap();

    (0..child_count)
        .map(|_| get_metadata_sum(nodes))
        .sum::<usize>()
        + nodes.take(metadata_count).sum::<usize>()
}

pub fn get_part_one(input: &str) -> usize {
    let mut nodes = input
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>());

    get_metadata_sum(&mut nodes)
}

fn get_root_node_value(nodes: &mut impl Iterator<Item = usize>) -> usize {
    let child_count = nodes.next().unwrap();
    let metadata_count = nodes.next().unwrap();

    match (child_count, metadata_count) {
        (0, metadata_count) => nodes.take(metadata_count).sum(),
        (child_count, metadata_count) => {
            let child_node_values = (0..child_count)
                .map(|_| get_root_node_value(nodes))
                .collect::<Vec<_>>();
            dbg!(&child_node_values);

            // "A metadata entry of 1 refers to the first child node…"
            nodes
                .take(metadata_count)
                .filter_map(|index| child_node_values.get(index - 1))
                .sum()
        }
    }
}

pub fn get_part_two(input: &str) -> usize {
    let mut nodes = input
        .trim()
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>());

    get_root_node_value(&mut nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"#;

    #[test]
    fn test_part_one() {
        assert_eq!(138, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(66, get_part_two(INPUT));
    }
}
