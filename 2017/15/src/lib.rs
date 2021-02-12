use std::str::FromStr;

const DIVISOR: usize = 2_147_483_647;

struct Generator {
    factor: usize,
    previous: usize,
    criterion: usize,
}

pub struct Generators {
    a: Generator,
    b: Generator,
}

impl FromStr for Generators {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines().collect::<Vec<_>>();
        let a_previous = get_starting_value(lines.get(0).unwrap());
        let b_previous = get_starting_value(lines.get(1).unwrap());

        Ok(Self {
            a: Generator {
                factor: 16807,
                previous: a_previous,
                criterion: 4,
            },
            b: Generator {
                factor: 48271,
                previous: b_previous,
                criterion: 8,
            },
        })
    }
}

impl Generators {
    pub fn get_judgement(&mut self, pairs: usize) -> usize {
        let mut count = 0;

        for _ in 0..pairs {
            let a_next = self.a.get_next() as u16;
            let b_next = self.b.get_next() as u16;

            if a_next == b_next {
                count += 1;
            }
        }

        count
    }

    pub fn get_picky_judgement(&mut self, pairs: usize) -> usize {
        let mut count = 0;

        for _ in 0..pairs {
            let a_next = self.a.get_next_meeting_criteria() as u16;
            let b_next = self.b.get_next_meeting_criteria() as u16;

            if a_next == b_next {
                count += 1;
            }
        }

        count
    }
}

impl Generator {
    fn get_next(&mut self) -> usize {
        let next = (self.previous * self.factor) % DIVISOR;
        self.previous = next;

        next
    }

    fn get_next_meeting_criteria(&mut self) -> usize {
        loop {
            let next = self.get_next();
            if next % self.criterion == 0 {
                return next;
            }
        }
    }
}

fn get_starting_value(line: &str) -> usize {
    line.trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_values() {
        let mut a = Generator {
            factor: 16807,
            previous: 65,
            criterion: 4,
        };
        let mut b = Generator {
            factor: 48271,
            previous: 8921,
            criterion: 8,
        };

        assert_eq!(1092455, a.get_next());
        assert_eq!(1181022009, a.get_next());
        assert_eq!(245556042, a.get_next());
        assert_eq!(1744312007, a.get_next());
        assert_eq!(1352636452, a.get_next());
        assert_eq!(430625591, b.get_next());
        assert_eq!(1233683848, b.get_next());
        assert_eq!(1431495498, b.get_next());
        assert_eq!(137874439, b.get_next());
        assert_eq!(285222916, b.get_next());
    }

    #[test]
    fn test_part_one_generate() {
        let mut generators = Generators {
            a: Generator {
                factor: 16807,
                previous: 65,
                criterion: 4,
            },
            b: Generator {
                factor: 48271,
                previous: 8921,
                criterion: 8,
            },
        };
        assert_eq!(1, generators.get_judgement(5));
    }

    #[test]
    fn test_part_two_values() {
        let mut a = Generator {
            factor: 16807,
            previous: 65,
            criterion: 4,
        };
        let mut b = Generator {
            factor: 48271,
            previous: 8921,
            criterion: 8,
        };

        assert_eq!(1352636452, a.get_next_meeting_criteria());
        assert_eq!(1992081072, a.get_next_meeting_criteria());
        assert_eq!(530830436, a.get_next_meeting_criteria());
        assert_eq!(1980017072, a.get_next_meeting_criteria());
        assert_eq!(740335192, a.get_next_meeting_criteria());
        assert_eq!(1233683848, b.get_next_meeting_criteria());
        assert_eq!(862516352, b.get_next_meeting_criteria());
        assert_eq!(1159784568, b.get_next_meeting_criteria());
        assert_eq!(1616057672, b.get_next_meeting_criteria());
        assert_eq!(412269392, b.get_next_meeting_criteria());
    }

    #[test]
    fn test_part_two_generate() {
        let mut generators = Generators {
            a: Generator {
                factor: 16807,
                previous: 65,
                criterion: 4,
            },
            b: Generator {
                factor: 48271,
                previous: 8921,
                criterion: 8,
            },
        };
        assert_eq!(0, generators.get_picky_judgement(4));
        assert_eq!(1, generators.get_picky_judgement(1056));
    }
}
