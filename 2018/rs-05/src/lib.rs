pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .chars()
        .fold(Vec::new(), |mut acc, c| {
            match acc.last() {
                Some(&previous) => {
                    if c.eq_ignore_ascii_case(&previous) && c != previous {
                        acc.pop();
                    } else {
                        acc.push(c);
                    }
                }
                None => acc.push(c),
            }
            acc
        })
        .len()
}

pub fn get_part_two(input: &str) -> usize {
    ('a'..='z')
        .map(|remove| {
            input
                .trim()
                .chars()
                .fold(Vec::new(), |mut acc, c| {
                    if !c.eq_ignore_ascii_case(&remove) {
                        match acc.last() {
                            Some(&previous) => {
                                if c.eq_ignore_ascii_case(&previous) && c != previous {
                                    acc.pop();
                                } else {
                                    acc.push(c);
                                }
                            }
                            None => acc.push(c),
                        }
                    }
                    acc
                })
                .len()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(polymer = {
        "aA",
        "abBA",
        "abAB",
        "aabAAB",
    }, length = {
        0,
        0,
        4,
        6,
    })]
    fn test_part_one(polymer: &str, length: usize) {
        assert_eq!(length, get_part_one(polymer));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, get_part_two("dabAcCaCBAcCcaDA"));
    }
}
