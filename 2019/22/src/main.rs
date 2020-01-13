use std::fs;

use regex::Regex;

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    fn new(count: usize) -> Self {
        let cards = (0..count).collect();
        Deck { cards }
    }

    fn get_position_for_card(&self, card: usize) -> usize {
        self.cards.iter().position(|&c| c == card).unwrap()
    }

    fn shuffle(&mut self, input: &str) {
        let deal_with_increment = Regex::new(r"deal with increment (\d+)").unwrap();
        let cut = Regex::new(r"cut (-?\d+)").unwrap();

        for line in input.lines() {
            match line {
                line if line.eq("deal into new stack") => {
                    self.deal_into_new_stack();
                }
                line if deal_with_increment.is_match(line) => {
                    let increment = deal_with_increment.captures(line).unwrap()[1]
                        .parse::<usize>()
                        .unwrap();
                    self.deal_with_increment_n(increment);
                }
                line if cut.is_match(line) => {
                    let cut = cut.captures(line).unwrap()[1].parse::<isize>().unwrap();
                    self.cut_n_cards(cut);
                }
                line => {
                    panic!("Unknown shuffle instruction: {}", line);
                }
            }
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.cards.reverse();
    }

    fn cut_n_cards(&mut self, n: isize) {
        let n = (self.cards.len() as isize + n) % self.cards.len() as isize;
        let n = n as usize;
        let mut cards = self.cards[n..].to_vec();
        cards.extend_from_slice(&self.cards[0..n]);
        self.cards = cards;
    }

    fn deal_with_increment_n(&mut self, n: usize) {
        self.cards.reverse();
        let mut cards = vec![0; self.cards.len()];
        let mut position = 0;

        while !self.cards.is_empty() {
            cards[position] = self.cards.pop().unwrap();
            position = (position + n) % cards.len();
        }

        self.cards = cards;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut deck = Deck::new(10007);
    deck.shuffle(&input.trim());
    println!(
        "…what is the position of card 2019? {}",
        deck.get_position_for_card(2019)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let deck = Deck::new(10);

        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], deck.cards);
    }
    #[test]
    fn test_deal_into_new_stack() {
        let mut deck = Deck::new(10);

        deck.deal_into_new_stack();

        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], deck.cards);
    }

    #[test]
    fn test_cut_n_cards() {
        let mut deck = Deck::new(10);

        deck.cut_n_cards(3);

        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], deck.cards);
    }

    #[test]
    fn test_cut_negative_n_cards() {
        let mut deck = Deck::new(10);

        deck.cut_n_cards(-4);

        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], deck.cards);
    }

    #[test]
    fn test_deal_with_increment_n() {
        let mut deck = Deck::new(10);

        deck.deal_with_increment_n(3);

        assert_eq!(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], deck.cards);
    }

    #[test]
    fn test_0369258147() {
        let mut deck = Deck::new(10);
        let instructions = r#"deal with increment 7
deal into new stack
deal into new stack"#;

        deck.shuffle(instructions);

        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], deck.cards);
    }

    #[test]
    fn test_3074185296() {
        let mut deck = Deck::new(10);
        let instructions = r#"cut 6
deal with increment 7
deal into new stack"#;

        deck.shuffle(instructions);

        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], deck.cards);
    }

    #[test]
    fn test_6307418529() {
        let mut deck = Deck::new(10);
        let instructions = r#"deal with increment 7
deal with increment 9
cut -2"#;

        deck.shuffle(instructions);

        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], deck.cards);
    }

    #[test]
    fn test_9258147036() {
        let mut deck = Deck::new(10);
        let instructions = r#"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"#;

        deck.shuffle(instructions);

        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6,], deck.cards);
    }
}
