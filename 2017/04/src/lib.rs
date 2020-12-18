use std::collections::HashSet;
use std::iter::FromIterator;

fn is_valid(passphrase: &str) -> bool {
    let words = passphrase.trim().split_whitespace().collect::<Vec<_>>();

    let distinct_words: HashSet<&str> = HashSet::from_iter(words.iter().cloned());

    words.len() == distinct_words.len()
}

pub fn get_valid_passphrase_count(input: &str) -> usize {
    input.trim().lines().filter(|line| is_valid(line)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(is_valid("aa bb cc dd ee"));
        assert!(!is_valid("aa bb cc dd aa"));
        assert!(is_valid("aa bb cc dd aaa"));
    }
}
