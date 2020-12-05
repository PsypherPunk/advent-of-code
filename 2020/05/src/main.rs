use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;

lazy_static! {
    static ref BINARY_SPACE_PARTITIONER: HashMap<char, char> = {
        let mut translation: HashMap<char, char> = HashMap::new();
        translation.insert('F', '0');
        translation.insert('B', '1');
        translation.insert('R', '1');
        translation.insert('L', '0');
        translation
    };
}

fn translate(seat: &str) -> String {
    seat.chars()
        .map(|char| BINARY_SPACE_PARTITIONER.get(&char).unwrap())
        .collect()
}

fn get_seat_id(seat: &str) -> usize {
    usize::from_str_radix(&translate(seat), 2).unwrap()
}

fn get_highest_seat_id(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| get_seat_id(line))
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the highest seat ID on a boarding pass? {}",
        get_highest_seat_id(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(357, get_seat_id("FBFBBFFRLR"));
        assert_eq!(567, get_seat_id("BFFFBBFRRR"));
        assert_eq!(119, get_seat_id("FFFBBBFRRR"));
        assert_eq!(820, get_seat_id("BBFFBBFRLL"));
    }
}
