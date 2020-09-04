use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROOM: Regex =
        Regex::new(r#"^(?P<encrypted_name>.+)-(?P<sector_id>\d+)\[(?P<checksum>\w{5})\]$"#)
            .unwrap();
}

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn from_str(input: &str) -> Self {
        let caps = ROOM.captures(&input).unwrap();
        Self {
            encrypted_name: caps["encrypted_name"].to_string(),
            sector_id: caps["sector_id"].parse::<usize>().unwrap(),
            checksum: caps["checksum"].to_string(),
        }
    }

    fn is_real(&self) -> bool {
        let mut chars = HashMap::new();
        self.encrypted_name.replace("-", "").chars().for_each(|c| {
            let count = chars.entry(c).or_insert(0);
            *count += 1;
        });

        let mut chars = chars.into_iter().collect::<Vec<(char, usize)>>();

        chars.sort_by(|(a_ch, a_count), (b_ch, b_count)| b_count.cmp(a_count).then(a_ch.cmp(b_ch)));

        self.checksum == chars.iter().map(|(ch, _)| ch).take(5).collect::<String>()
    }
}

fn get_sector_id_sum(input: &str) -> usize {
    input
        .lines()
        .map(|line| Room::from_str(&line))
        .filter(|room| room.is_real())
        .map(|room| room.sector_id)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of the sector IDs of the real rooms? {}",
        get_sector_id_sum(&input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abxyz() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let room = Room::from_str(&input);

        assert_eq!(true, room.is_real());
    }

    #[test]
    fn test_abcde() {
        let input = "a-b-c-d-e-f-g-h-987[abcde]";
        let room = Room::from_str(&input);

        assert_eq!(true, room.is_real());
    }

    #[test]
    fn test_oarel() {
        let input = "not-a-real-room-404[oarel]";
        let room = Room::from_str(&input);

        assert_eq!(true, room.is_real());
    }

    #[test]
    fn test_decoy() {
        let input = "totally-real-room-200[decoy]";
        let room = Room::from_str(&input);

        assert_eq!(false, room.is_real());
    }
}
