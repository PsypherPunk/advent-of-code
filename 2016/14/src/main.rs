use std::fs;

use md5::{Digest, Md5};

#[derive(Debug)]
struct Key {
    hash: String,
    index: usize,
}

fn next_1000_has_five_in_a_row(start: &usize, character: &char, salt: &[u8]) -> bool {
    for index in (start + 1)..=(start + 1_000) {
        let mut hasher = Md5::new();
        hasher.update(salt);
        hasher.update(index.to_string());
        let result = &hasher.finalize()[..];

        let hex_string = hex::encode(result);
        let five_in_a_row = hex_string
            .chars()
            .collect::<Vec<char>>()
            .windows(5)
            .any(|window| {
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

fn get_keys(input: &str) -> Vec<Key> {
    let mut keys = Vec::with_capacity(64);
    let salt = input.trim().as_bytes();
    let mut index = 0;

    while keys.len() < 64 {
        let mut hasher = Md5::new();
        hasher.update(salt);
        hasher.update(index.to_string());
        let result = &hasher.finalize()[..];

        let hex_string = hex::encode(result);
        let mut three_in_a_row = hex_string
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .filter(|window| window[0] == window[1] && window[1] == window[2])
            .take(1)
            .map(|window| window[0])
            .collect::<Vec<char>>();
        three_in_a_row.dedup();

        if three_in_a_row
            .iter()
            .any(|character| next_1000_has_five_in_a_row(&index, character, salt))
        {
            keys.push(Key {
                hash: hex_string,
                index,
            });
        }

        index += 1;
    }

    keys
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what index produces your 64th one-time pad key? {}",
        get_keys(&input).last().unwrap().index,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "abc";

        let keys = get_keys(&input);

        assert_eq!(22728, keys.get(63).unwrap().index);
    }
}
