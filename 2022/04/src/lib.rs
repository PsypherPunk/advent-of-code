use std::str::FromStr;

struct Assignment {
    start: usize,
    end: usize,
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| format!("could not split on hyphen: {}", s))?;

        Ok(Self {
            start: start
                .parse()
                .map_err(|e| format!("invalid number: {}", e))?,
            end: end.parse().map_err(|e| format!("invalid number: {}", e))?,
        })
    }
}

struct Pair {
    first: Assignment,
    second: Assignment,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(',')
            .ok_or_else(|| format!("could not split on comma: {}", s))?;

        Ok(Self {
            first: Assignment::from_str(first)?,
            second: Assignment::from_str(second)?,
        })
    }
}

impl Pair {
    fn contains(&self) -> bool {
        self.first.start <= self.second.start && self.first.end >= self.second.end
            || self.second.start <= self.first.start && self.second.end >= self.first.end
    }

    fn overlaps(&self) -> bool {
        (self.first.start <= self.second.start && self.second.start <= self.first.end)
            || (self.second.start <= self.first.start && self.first.start <= self.second.end)
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    input
        .trim()
        .lines()
        .map(Pair::from_str)
        .try_fold(0, |mut acc, pair| {
            acc += match pair?.contains() {
                true => 1,
                false => 0,
            };

            Ok(acc)
        })
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    input
        .trim()
        .lines()
        .map(Pair::from_str)
        .try_fold(0, |mut acc, pair| {
            acc += match pair?.overlaps() {
                true => 1,
                false => 0,
            };

            Ok(acc)
        })
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
        assert_eq!(Ok(2), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(4), get_part_two(INPUT));
    }
}
