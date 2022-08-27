fn get_positions(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .flat_map(|pos| pos.parse::<isize>())
        .collect()
}

pub fn get_part_one(input: &str) -> isize {
    let positions = get_positions(input);

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    (*min..=*max)
        .map(|alignment| {
            positions
                .iter()
                .map(|position| (position - alignment).abs())
                .sum()
        })
        .min()
        .unwrap()
}

pub fn get_part_two(input: &str) -> isize {
    let positions = get_positions(input);

    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    (*min..=*max)
        .map(|alignment| {
            positions
                .iter()
                .flat_map(|position| {
                    0..=(position - alignment).abs()
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_one() {
        assert_eq!(37, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(168, get_part_two(INPUT));
    }
}
