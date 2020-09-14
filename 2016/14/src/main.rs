use std::collections::HashMap;
use std::fs;

use md5::{Digest, Md5};

#[derive(Debug)]
struct Key {
    hash: String,
    index: usize,
}

struct OneTimePad {
    keys: Vec<Key>,
    salt: String,
    stretch: bool,
    cache: HashMap<usize, String>,
}

impl OneTimePad {
    fn from_string(salt: &str) -> Self {
        Self {
            keys: Vec::new(),
            salt: salt.trim().to_string(),
            stretch: false,
            cache: HashMap::new(),
        }
    }

    fn get_key_at(&mut self, index: usize) -> String {
        let salt = self.salt.as_bytes();
        let stretch = self.stretch;

        let key = self.cache.entry(index).or_insert_with(|| {
            let mut hasher = Md5::new();
            hasher.update(salt);
            hasher.update(index.to_string());
            let result = &hasher.finalize()[..];
            let mut key = hex::encode(result);

            if stretch {
                for _ in 0..2016 {
                    let mut hasher = Md5::new();
                    hasher.update(key.clone());
                    let result = &hasher.finalize()[..];
                    key = hex::encode(result);
                }
            }
            key
        });

        key.clone()
    }

    fn next_1000_has_five_in_a_row(&mut self, start: &usize, character: &char) -> bool {
        for index in (start + 1)..=(start + 1_000) {
            let key = self.get_key_at(index);

            let five_in_a_row = key.chars().collect::<Vec<char>>().windows(5).any(|window| {
                window[0] == *character
                    && window[0] == window[1]
                    && window[1] == window[2]
                    && window[2] == window[3]
                    && window[3] == window[4]
            });

            if five_in_a_row {
                return true;
            }
        }

        false
    }

    fn calculate_keys(&mut self, count: usize) {
        let mut index = 0;

        while self.keys.len() < count {
            let key = self.get_key_at(index);
            let three_in_a_row = key
                .chars()
                .collect::<Vec<char>>()
                .windows(3)
                .filter(|window| window[0] == window[1] && window[1] == window[2])
                .take(1)
                .map(|window| window[0])
                .collect::<Vec<char>>();

            if three_in_a_row
                .iter()
                .any(|character| self.next_1000_has_five_in_a_row(&index, character))
            {
                self.keys.push(Key { hash: key, index });
            }

            index += 1;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut one_time_pad = OneTimePad::from_string(&input);
    one_time_pad.calculate_keys(64);

    println!(
        "…what index produces your 64th one-time pad key? {}",
        one_time_pad.keys.get(63).unwrap().index,
    );

    let mut one_time_pad = OneTimePad::from_string(&input);
    one_time_pad.stretch = true;
    one_time_pad.calculate_keys(64);

    println!(
        "…what index now produces your 64th one-time pad key? {}",
        one_time_pad.keys.get(63).unwrap().index,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "abc";

        let mut one_time_pad = OneTimePad::from_string(&input);
        one_time_pad.calculate_keys(64);

        assert_eq!(22728, one_time_pad.keys.get(63).unwrap().index);
    }

    #[test]
    fn test_part_two() {
        let input = "abc";

        let mut one_time_pad = OneTimePad::from_string(&input);
        one_time_pad.stretch = true;
        one_time_pad.calculate_keys(64);

        assert_eq!(22551, one_time_pad.keys.get(63).unwrap().index);
    }
}
