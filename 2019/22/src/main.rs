use std::fs;

use mod_exp::mod_exp;
use regex::Regex;

struct Deck {
    count: i128,
}

#[derive(Copy, Clone)]
enum Technique {
    DealIntoNewStack,
    CutNCards(i128),
    DealWithIncrement(i128),
}

impl Deck {
    fn new(count: i128) -> Self {
        Deck { count }
    }

    fn shuffle(&self, input: &str, card: i128) -> i128 {
        let techniques = self.get_techniques(input);

        techniques.iter().fold(card, |position, &cmd| match cmd {
            Technique::DealIntoNewStack => self.count - 1 - position,
            Technique::CutNCards(n) => (((position - n) % self.count) + self.count) % self.count,
            Technique::DealWithIncrement(n) => (position * n) % self.count,
        })
    }

    /// Purloined wholeheartedly from here:
    /// https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/
    fn shuffle_repeatedly(&self, input: &str, repetitions: i128, position: i128) -> i128 {
        let techniques = self.get_techniques(input);

        let (increment, offset) = techniques.iter().rev().fold((1, 0), |(i, o), &technique| {
            let (i, o) = match technique {
                Technique::DealIntoNewStack => (-i, -o - 1),
                Technique::CutNCards(n) => (i, o + n),
                Technique::DealWithIncrement(n) => {
                    let n = mod_exp(n, self.count - 2, self.count);
                    (i * n, o * n)
                }
            };
            (i % self.count, o % self.count)
        });

        let term1 = position * mod_exp(increment, repetitions, self.count) % self.count;
        let tmp = (mod_exp(increment, repetitions, self.count) - 1)
            * mod_exp(increment - 1, self.count - 2, self.count)
            % self.count;
        let term2 = offset * tmp % self.count;
        (term1 + term2) % self.count
    }

    fn get_techniques(&self, input: &str) -> Vec<Technique> {
        let deal_with_increment = Regex::new(r"deal with increment (\d+)").unwrap();
        let cut = Regex::new(r"cut (-?\d+)").unwrap();

        input
            .lines()
            .map(|line| match line {
                line if line.eq("deal into new stack") => Technique::DealIntoNewStack,
                line if deal_with_increment.is_match(line) => {
                    let increment = deal_with_increment.captures(line).unwrap()[1]
                        .parse::<i128>()
                        .unwrap();
                    Technique::DealWithIncrement(increment)
                }
                line if cut.is_match(line) => {
                    let cut = cut.captures(line).unwrap()[1].parse::<i128>().unwrap();
                    Technique::CutNCards(cut)
                }
                line => {
                    panic!("Unknown shuffle instruction: {}", line);
                }
            })
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let deck = Deck::new(10007);
    println!(
        "…what is the position of card 2019? {}",
        deck.shuffle(&input, 2019),
    );

    let deck = Deck::new(119_315_717_514_047_i128);
    println!(
        "…what number is on the card that ends up in position 2020? {}",
        deck.shuffle_repeatedly(&input, 101_741_582_076_661_i128, 2020),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        let deck = Deck::new(10);

        // 9 8 7 6 5 4 3 2 1 0
        assert_eq!(9, deck.shuffle("deal into new stack", 0));
        assert_eq!(0, deck.shuffle("deal into new stack", 9));
    }

    #[test]
    fn test_cut_n_cards() {
        let deck = Deck::new(10);

        // 3 4 5 6 7 8 9 0 1 2
        assert_eq!(7, deck.shuffle("cut 3", 0));
        assert_eq!(0, deck.shuffle("cut 3", 3));
    }

    #[test]
    fn test_cut_negative_n_cards() {
        let deck = Deck::new(10);

        // 6 7 8 9 0 1 2 3 4 5
        assert_eq!(7, deck.shuffle("cut -4", 3));
        assert_eq!(0, deck.shuffle("cut -4", 6));
    }

    #[test]
    fn test_deal_with_increment_n() {
        let deck = Deck::new(10);

        // 0 7 4 1 8 5 2 9 6 3
        assert_eq!(0, deck.shuffle("deal with increment 3", 0));
        assert_eq!(9, deck.shuffle("deal with increment 3", 3));
    }

    #[test]
    fn test_0369258147() {
        let deck = Deck::new(10);
        let instructions = r#"deal with increment 7
deal into new stack
deal into new stack"#;

        // 0 3 6 9 2 5 8 1 4 7
        assert_eq!(0, deck.shuffle(&instructions, 0));
        assert_eq!(1, deck.shuffle(&instructions, 3));
        assert_eq!(9, deck.shuffle(&instructions, 7));
    }

    #[test]
    fn test_3074185296() {
        let deck = Deck::new(10);
        let instructions = r#"cut 6
deal with increment 7
deal into new stack"#;

        // 3 0 7 4 1 8 5 2 9 6
        assert_eq!(0, deck.shuffle(&instructions, 3));
        assert_eq!(1, deck.shuffle(&instructions, 0));
        assert_eq!(9, deck.shuffle(&instructions, 6));
    }

    #[test]
    fn test_6307418529() {
        let deck = Deck::new(10);
        let instructions = r#"deal with increment 7
deal with increment 9
cut -2"#;

        // 6 3 0 7 4 1 8 5 2 9
        assert_eq!(0, deck.shuffle(&instructions, 6));
        assert_eq!(1, deck.shuffle(&instructions, 3));
        assert_eq!(9, deck.shuffle(&instructions, 9));
    }

    #[test]
    fn test_9258147036() {
        let deck = Deck::new(10);
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

        // 9 2 5 8 1 4 7 0 3 6
        assert_eq!(0, deck.shuffle(&instructions, 9));
        assert_eq!(1, deck.shuffle(&instructions, 2));
        assert_eq!(9, deck.shuffle(&instructions, 6));
    }
}
