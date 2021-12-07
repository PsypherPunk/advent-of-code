pub fn get_part_one(input: &str) -> isize {
    let positions = input
        .trim()
        .split(',')
        .flat_map(|pos| pos.parse::<isize>())
        .collect::<Vec<_>>();

    (0..positions.len())
        .map(|alignment| {
            positions
                .iter()
                .map(|position| (position - alignment as isize).abs())
                .sum::<isize>()
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
        assert_eq!(1, 2)
    }
}
