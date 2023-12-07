use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    bid: usize,
    r#type: Type,
}

#[derive(Debug, Eq, PartialEq)]
struct JokerHand {
    cards: Vec<u32>,
    bid: usize,
    r#type: Type,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.r#type.cmp(&other.r#type) {
            Ordering::Equal => {
                match self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(a, b)| match a.cmp(b) {
                        Ordering::Equal => None,
                        o => Some(o),
                    }) {
                    Some(o) => o,
                    _ => Ordering::Equal,
                }
            }
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.r#type.cmp(&other.r#type) {
            Ordering::Equal => {
                match self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(a, b)| match a.cmp(b) {
                        Ordering::Equal => None,
                        o => Some(o),
                    }) {
                    Some(o) => o,
                    _ => Ordering::Equal,
                }
            }
            o => o,
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').ok_or(format!("bad line: {}", s))?;

        let bid = bid.parse::<usize>().map_err(|e| e.to_string())?;

        let cards = cards
            .chars()
            .filter_map(|c| match c {
                'A' => Some(14),
                'K' => Some(13),
                'Q' => Some(12),
                'J' => Some(11),
                'T' => Some(10),
                _ => c.to_digit(10),
            })
            .collect::<Vec<_>>();

        let counts = cards.iter().fold(BTreeMap::new(), |mut counts, card| {
            *counts.entry(card).or_insert(0) += 1;

            counts
        });
        let mut counts = counts.into_values().collect::<Vec<_>>();
        counts.sort_unstable();
        let r#type = match counts.pop().ok_or("bad counts".to_owned())? {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => match counts.pop().ok_or("bad count".to_owned())? {
                2 => Type::FullHouse,
                _ => Type::ThreeOfAKind,
            },
            2 => match counts.pop().ok_or("bad count".to_owned())? {
                2 => Type::TwoPair,
                _ => Type::Pair,
            },
            1 => Type::HighCard,
            _ => unreachable!(),
        };

        Ok(Self { cards, bid, r#type })
    }
}

impl FromStr for JokerHand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.trim().split_once(' ').ok_or(format!("bad line: {}", s))?;

        let bid = bid.parse::<usize>().map_err(|e| e.to_string())?;

        let cards = cards
            .chars()
            .filter_map(|c| match c {
                'A' => Some(14),
                'K' => Some(13),
                'Q' => Some(12),
                'J' => Some(1),
                'T' => Some(10),
                _ => c.to_digit(10),
            })
            .collect::<Vec<_>>();

        let jokers = cards.iter().filter(|&card| *card == 1).count();
        let counts =
            cards
                .iter()
                .filter(|&card| *card != 1)
                .fold(BTreeMap::new(), |mut counts, card| {
                    *counts.entry(card).or_insert(0) += 1;

                    counts
                });
        let mut counts = counts.into_values().collect::<Vec<_>>();
        counts.sort_unstable();

        let r#type = match counts.pop().unwrap_or(0) + jokers {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => match counts.pop().ok_or("bad count".to_owned())? {
                2 => Type::FullHouse,
                _ => Type::ThreeOfAKind,
            },
            2 => match counts.pop().ok_or("bad count".to_owned())? {
                2 => Type::TwoPair,
                _ => Type::Pair,
            },
            1 => Type::HighCard,
            _ => unreachable!(),
        };

        Ok(Self { cards, bid, r#type })
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut hands = input
        .trim()
        .lines()
        .map(Hand::from_str)
        .collect::<Result<Vec<_>, String>>()?;

    hands.sort_unstable();

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    Ok(winnings)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut hands = input
        .trim()
        .lines()
        .map(JokerHand::from_str)
        .collect::<Result<Vec<_>, String>>()?;

    hands.sort_unstable();

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    Ok(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(6440), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(5905), get_part_two(INPUT));
    }
}
