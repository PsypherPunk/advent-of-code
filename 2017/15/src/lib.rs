use std::str::FromStr;

const DIVISOR: usize = 2_147_483_647;

struct Generator {
    factor: usize,
    previous: usize,
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
            },
            b: Generator {
                factor: 48271,
                previous: b_previous,
            },
        })
    }
}

impl Generators {
    pub fn get_judgement(&mut self, pairs: usize) -> usize {
        let mut count = 0;

        for _ in 0..pairs {
            let a_next = self.a.next() as u16;
            let b_next = self.b.next() as u16;

            if a_next == b_next {
                count += 1;
            }
        }

        count
    }
}

impl Generator {
    fn next(&mut self) -> usize {
        let next = (self.previous * self.factor) % DIVISOR;
        self.previous = next;

        next
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
        };
        let mut b = Generator {
            factor: 48271,
            previous: 8921,
        };

        assert_eq!(1092455, a.next());
        assert_eq!(1181022009, a.next());
        assert_eq!(245556042, a.next());
        assert_eq!(1744312007, a.next());
        assert_eq!(1352636452, a.next());
        assert_eq!(430625591, b.next());
        assert_eq!(1233683848, b.next());
        assert_eq!(1431495498, b.next());
        assert_eq!(137874439, b.next());
        assert_eq!(285222916, b.next());
    }

    #[test]
    fn test_part_one_generate() {
        let mut generators = Generators {
            a: Generator {
                factor: 16807,
                previous: 65,
            },
            b: Generator {
                factor: 48271,
                previous: 8921,
            },
        };
        assert_eq!(1, generators.get_judgement(5));
    }
}
