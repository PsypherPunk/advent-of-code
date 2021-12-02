fn get_parsed_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .flat_map(|line| line.parse::<usize>())
        .collect()
}

pub fn get_larger_measurements(input: &str) -> usize {
    get_parsed_input(input)
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

// We could compare these using `.windows(3)`. However, as we're
// comparing `a + b + c < b + c + d` we can factor out `b + c` and
// simply compare `a < d`.
pub fn get_larger_sums(input: &str) -> usize {
    let depths = get_parsed_input(input);
    depths
        .iter()
        .zip(depths.iter().skip(3))
        .filter(|(a, d)| d > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(7, get_larger_measurements(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(5, get_larger_sums(INPUT));
    }
}
