use std::str::FromStr;

#[derive(Clone)]
struct Value {
    number: usize,
    depth: usize,
}

#[derive(Clone)]
struct SnailfishNumber {
    values: Vec<Value>,
}

impl FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut snailfish_numbers = SnailfishNumber { values: Vec::new() };

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ']' => {
                    depth -= 1;
                }
                ',' => (),
                d => {
                    snailfish_numbers.values.push(Value {
                        number: d.to_digit(10).ok_or_else(|| "invalid digit".to_owned())? as usize,
                        depth: depth - 1,
                    });
                }
            }
        }

        Ok(snailfish_numbers)
    }
}

impl SnailfishNumber {
    fn splode(&mut self) -> bool {
        let mut sploded = false;

        for i in 0..self.values.len() {
            if self.values[i].depth < 4 {
                continue;
            }

            if i > 0 {
                self.values[i - 1].number += self.values[i].number;
            }

            if i + 2 < self.values.len() {
                self.values[i + 2].number += self.values[i + 1].number;
            }

            self.values[i].number = 0;
            self.values[i].depth -= 1;
            self.values.remove(i + 1);

            sploded = true;
            break;
        }

        sploded
    }

    fn split(&mut self) -> bool {
        let mut splitted = false;

        for i in 0..self.values.len() {
            if self.values[i].number < 10 {
                continue;
            }

            let (a, b) = (
                self.values[i].number / 2,
                (self.values[i].number / 2) + self.values[i].number % 2,
            );

            self.values[i].number = a;
            self.values[i].depth += 1;
            self.values.insert(
                i + 1,
                Value {
                    number: b,
                    depth: self.values[i].depth,
                },
            );

            splitted = true;
            break;
        }

        splitted
    }

    fn reduce(&mut self) {
        loop {
            if self.splode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn add(&mut self, other: SnailfishNumber) {
        self.values.extend(other.values);
        for i in 0..self.values.len() {
            self.values[i].depth += 1;
        }
    }

    fn get_magnitude(&self) -> usize {
        let mut values = self.values.clone();

        while values.len() > 1 {
            for i in 0..(values.len() - 1) {
                if values[i].depth == values[i + 1].depth {
                    values[i].number = (3 * values[i].number) + (2 * values[i + 1].number);
                    values.remove(i + 1);

                    if values[i].depth > 0 {
                        values[i].depth -= 1;
                    }

                    break;
                }
            }
        }

        values[0].number
    }
}

pub fn get_part_one(input: &str) -> usize {
    let mut lines = input.trim().lines();

    let init = SnailfishNumber::from_str(lines.next().unwrap()).unwrap();

    lines
        .flat_map(SnailfishNumber::from_str)
        .fold(init, |mut acc, next| {
            acc.add(next);
            acc.reduce();

            acc
        })
        .get_magnitude()
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use parameterized::parameterized;

    const INPUT: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"#;

    #[parameterized(snailfish_number = {
        "[[1,2],[[3,4],5]]",
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    }, magnitude = {
        143, 1384, 445, 791, 1137, 3488,
    })]
    fn test_get_magnitude(snailfish_number: &str, magnitude: usize) {
        let snailfish_number = SnailfishNumber::from_str(snailfish_number).unwrap();

        assert_eq!(magnitude, snailfish_number.get_magnitude());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(4140, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
