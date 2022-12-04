pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let (first, second) = line.split_once(',').unwrap();

            let (first_start, first_end) = first.split_once('-').unwrap();
            let (second_start, second_end) = second.split_once('-').unwrap();

            let (first_start, first_end) = (
                first_start.parse::<usize>().unwrap(),
                first_end.parse::<usize>().unwrap(),
            );
            let (second_start, second_end) = (
                second_start.parse::<usize>().unwrap(),
                second_end.parse::<usize>().unwrap(),
            );

            first_start <= second_start && first_end >= second_end
                || second_start <= first_start && second_end >= first_end
        })
        .count()
}

pub fn get_part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| {
            let (first, second) = line.split_once(',').unwrap();

            let (first_start, first_end) = first.split_once('-').unwrap();
            let (second_start, second_end) = second.split_once('-').unwrap();

            let (first_start, first_end) = (
                first_start.parse::<usize>().unwrap(),
                first_end.parse::<usize>().unwrap(),
            );
            let (second_start, second_end) = (
                second_start.parse::<usize>().unwrap(),
                second_end.parse::<usize>().unwrap(),
            );

            (first_start <= second_start && second_start <= first_end)
                || (second_start <= first_start && first_start <= second_end)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(2, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, get_part_two(INPUT));
    }
}
