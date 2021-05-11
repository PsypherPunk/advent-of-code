use std::collections::HashSet;

fn is_valid(passphrase: &str) -> bool {
    let words = passphrase.trim().split_whitespace().collect::<Vec<_>>();

    let distinct_words: HashSet<&str> = words.iter().cloned().collect();

    words.len() == distinct_words.len()
}

fn is_valid_non_anagram_passphrase(passphrase: &str) -> bool {
    let words = passphrase.trim().split_whitespace().collect::<Vec<_>>();

    let distinct_non_anagrams: HashSet<String> = words
        .iter()
        .map(|word| {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort_unstable();
            chars.iter().collect::<String>()
        })
        .collect::<HashSet<_>>();

    words.len() == distinct_non_anagrams.len()
}

pub fn get_valid_passphrase_count(input: &str) -> usize {
    input.trim().lines().filter(|line| is_valid(line)).count()
}

pub fn get_valid_non_anagram_passphrase_count(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| is_valid_non_anagram_passphrase(line))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert!(is_valid("aa bb cc dd ee"));
        assert!(!is_valid("aa bb cc dd aa"));
        assert!(is_valid("aa bb cc dd aaa"));
    }

    #[test]
    fn test_part_two() {
        assert!(is_valid_non_anagram_passphrase("abcde fghij"));
        assert!(!is_valid_non_anagram_passphrase("abcde xyz ecdab"));
        assert!(is_valid_non_anagram_passphrase("a ab abc abd abf abj"));
        assert!(is_valid_non_anagram_passphrase("iiii oiii ooii oooi oooo"));
        assert!(!is_valid_non_anagram_passphrase("oiii ioii iioi iiio"));
    }
}
