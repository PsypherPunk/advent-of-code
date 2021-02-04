use std::collections::HashMap;

fn get_scanners(input: &str) -> HashMap<usize, usize> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (depth, range) = match line.trim().split(':').collect::<Vec<_>>()[..] {
                [depth, range] => (depth, range),
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            };

            (
                depth.trim().parse::<usize>().unwrap(),
                range.trim().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn get_position_for_range_after(range: usize, picoseconds: usize) -> usize {
    let offset = picoseconds % ((range - 1) * 2);

    match offset > range - 1 {
        true => 2 * (range - 1) - offset,
        false => offset,
    }
}

pub fn get_severity(input: &str) -> usize {
    get_scanners(&input)
        .iter()
        .filter(|&(depth, range)| get_position_for_range_after(*range, *depth) == 0)
        .map(|(depth, range)| depth * range)
        .sum()
}

pub fn get_delay(input: &str) -> usize {
    let scanners = get_scanners(&input);

    (0..)
        .find(|delay| {
            scanners
                .iter()
                .find(|&(depth, range)| get_position_for_range_after(*range, depth + delay) == 0)
                .is_none()
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"0: 3
1: 2
4: 4
6: 4"#;

        assert_eq!(0, get_position_for_range_after(3, 0));
        assert_eq!(1, get_position_for_range_after(3, 1));
        assert_eq!(2, get_position_for_range_after(3, 2));
        assert_eq!(1, get_position_for_range_after(3, 3));
        assert_eq!(0, get_position_for_range_after(3, 4));
        assert_eq!(1, get_position_for_range_after(3, 5));

        assert_eq!(24, get_severity(&input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"0: 3
1: 2
4: 4
6: 4"#;

        assert_eq!(10, get_delay(&input));
    }
}
