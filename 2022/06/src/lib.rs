use std::collections::HashSet;

pub fn get_part_one(input: &str) -> usize {
    let stream = input.trim().chars().collect::<Vec<_>>();

    let (offset, _) = stream
        .windows(4)
        .enumerate()
        .find(|(_, marker)| marker.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap();

    offset + 4
}

pub fn get_part_two(input: &str) -> usize {
    let stream = input.trim().chars().collect::<Vec<_>>();

    let (offset, _) = stream
        .windows(14)
        .enumerate()
        .find(|(_, marker)| marker.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap();

    offset + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    use parameterized::parameterized;

    #[parameterized(stream = {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    }, offset = {
        7, 5, 6, 10, 11
    })]
    fn test_part_one(stream: &str, offset: usize) {
        assert_eq!(offset, get_part_one(stream));
    }

    #[parameterized(stream = {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    }, offset = {
        19, 23, 23, 29, 26
    })]
    fn test_part_two(stream: &str, offset: usize) {
        assert_eq!(offset, get_part_two(stream));
    }
}
