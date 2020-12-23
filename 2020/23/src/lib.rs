use std::cmp::Ordering;

pub struct CrabCups {
    cups: Vec<usize>,
    current_cup: usize,
}

impl CrabCups {
    pub fn from_str(s: &str, num_cups: usize) -> Self {
        let cups = s
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        let mut pointers = (1..=num_cups + 1).collect::<Vec<_>>();
        pointers[num_cups] = *cups.get(0).unwrap();
        cups.windows(2).for_each(|window| match window {
            [a, b] => pointers[*a] = *b,
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        });
        pointers[*cups.last().unwrap()] = *cups.get(0).unwrap();
        if num_cups > cups.len() {
            pointers[*cups.last().unwrap()] = cups.len() + 1;
        }

        Self {
            cups: pointers,
            current_cup: *cups.get(0).unwrap(),
        }
    }

    fn dereference_pointers(&self, start: usize, limit: usize) -> Vec<usize> {
        let mut pointer = self.cups[start];

        (1..=limit)
            .map(|_| {
                let cup = pointer;
                pointer = self.cups[cup];

                cup
            })
            .collect::<Vec<_>>()
    }

    pub fn get_labels(&self) -> String {
        self.dereference_pointers(1, self.cups.len() - 2)
            .iter()
            .map(|cup| cup.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    fn make_move(&mut self) {
        let three_cups = self.dereference_pointers(self.current_cup, 3);

        self.cups[self.current_cup] = self.cups[*three_cups.last().unwrap()];

        let mut destination_cup = match self.current_cup.cmp(&1) {
            Ordering::Greater => self.current_cup - 1,
            _ => self.cups.len() - 1,
        };
        while three_cups.contains(&destination_cup) {
            destination_cup = match destination_cup.cmp(&1) {
                Ordering::Greater => destination_cup - 1,
                _ => self.cups.len() - 1,
            };
        }

        self.cups[*three_cups.last().unwrap()] = self.cups[destination_cup];
        self.cups[destination_cup] = *three_cups.first().unwrap();

        self.current_cup = self.cups[self.current_cup];
    }

    pub fn make_moves(&mut self, moves: usize) {
        for _ in 0..moves {
            self.make_move();
        }
    }

    pub fn get_cup_product_two_clockwise_of_one(&self) -> usize {
        let cups = self.dereference_pointers(1, 2);

        cups.get(0).unwrap() * cups.get(1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"389125467"#;

    #[test]
    fn test_part_one() {
        let mut crab_cups = CrabCups::from_str(&INPUT, INPUT.trim().len());

        crab_cups.make_moves(100);

        assert_eq!("67384529", crab_cups.get_labels());
    }

    #[test]
    fn test_part_two() {
        let mut crab_cups = CrabCups::from_str(&INPUT, 1_000_000);

        crab_cups.make_moves(10_000_000);

        assert_eq!(
            149245887792,
            crab_cups.get_cup_product_two_clockwise_of_one(),
        );
    }
}
