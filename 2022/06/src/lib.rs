use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidStreamError(String),
}

fn find_marker(input: &str, length: usize) -> Result<usize, AdventOfCodeError> {
    let (offset, _) = input
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_, marker)| marker.iter().collect::<HashSet<_>>().len() == length)
        .ok_or_else(|| AdventOfCodeError::InvalidStreamError(input.to_owned()))?;

    Ok(offset + length)
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    find_marker(input, 4)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    find_marker(input, 14)
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
        assert_eq!(Ok(offset), get_part_one(stream));
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
        assert_eq!(Ok(offset), get_part_two(stream));
    }
}
