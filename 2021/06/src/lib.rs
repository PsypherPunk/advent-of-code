pub fn get_part_one(input: &str) -> usize {
    let mut lanternfish = input
        .trim()
        .split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        for i in 0..lanternfish.len() {
            match lanternfish[i] {
                0 => {
                    lanternfish.push(8);
                    lanternfish[i] = 6;
                }
                _ => lanternfish[i] -= 1,
            }
        }
    }

    lanternfish.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part_one() {
        assert_eq!(5934, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, 2)
    }
}
