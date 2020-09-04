use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROOM: Regex =
        Regex::new(r#"^(?P<encrypted_name>.+)-(?P<sector_id>\d+)\[(?P<checksum>\w{5})\]$"#)
            .unwrap();
}

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

    fn get_decrypted_name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|ch| {
                let a = b'a';
                let shift = (&self.sector_id % 26) as u8;
                match ch {
                    '-' => ' ',
                    _ => (((ch as u8 - a + shift) % 26) + a) as char,
                }
            })
            .collect()
    }
}

fn get_north_pole_objects_room(input: &str) -> Room {
    input
        .lines()
        .map(|line| Room::from_str(&line))
        .find(|room| room.is_real() && room.get_decrypted_name().contains("pole"))
        .unwrap()
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
    );

    println!(
        "What is the sector ID of the room where North Pole objects are stored? {}",
        get_north_pole_objects_room(&input).sector_id,
    );
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

    #[test]
    fn test_very_encrypted_name() {
        let room = Room {
            encrypted_name: "qzmt-zixmtkozy-ivhz".to_string(),
            sector_id: 343,
            checksum: "".to_string(),
        };

        assert_eq!("very encrypted name", room.get_decrypted_name());
    }
}
