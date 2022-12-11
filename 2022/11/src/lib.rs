#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::VecDeque;

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidMonkeyError(ParseError<LineCol>),
    InsufficientMonkeysError(usize),
}

impl From<ParseError<LineCol>> for AdventOfCodeError {
    fn from(error: ParseError<LineCol>) -> Self {
        AdventOfCodeError::InvalidMonkeyError(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Squared,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test_divisible_by: usize,
    true_target: usize,
    false_target: usize,
    inspected_count: usize,
}

impl Monkey {
    fn get_target(&mut self) -> Option<(usize, usize)> {
        match self.items.pop_front() {
            None => None,
            Some(worry) => {
                self.inspected_count += 1;

                let worry = match self.operation {
                    Operation::Multiply(n) => worry * n,
                    Operation::Add(n) => worry + n,
                    Operation::Squared => worry * worry,
                } / 3;
                let target = match worry % self.test_divisible_by {
                    0 => self.true_target,
                    _ => self.false_target,
                };

                Some((worry, target))
            }
        }
    }

    fn get_worrisome_target(&mut self, least_common_multiple: usize) -> Option<(usize, usize)> {
        match self.items.pop_front() {
            None => None,
            Some(worry) => {
                self.inspected_count += 1;

                let worry = match self.operation {
                    Operation::Multiply(n) => worry * n,
                    Operation::Add(n) => worry + n,
                    Operation::Squared => worry * worry,
                } % least_common_multiple;
                let target = match worry % self.test_divisible_by {
                    0 => self.true_target,
                    _ => self.false_target,
                };

                Some((worry, target))
            }
        }
    }
}

peg::parser! {
    grammar monkeys() for str {
        rule _() = [' ' | '\n']*

        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule operation() -> Operation
            = s:$("* old" / "*" / "+") _ n:number()? {?
                match s {
                    "* old" => Ok(Operation::Squared),
                    "+" => match n {
                            Some(n) => Ok(Operation::Add(n)),
                            None => Err("operation()"),
                        },
                    "*" => match n {
                            Some(n) => Ok(Operation::Multiply(n)),
                            None => Err("operation()"),
                        _ => Err("operation()"),
                    }
                    _ => Err("operation()"),
                }
            }

        rule monkey() -> Monkey
            = "Monkey" _ number() ":"
              _
              "Starting items:" _ items:number() ++ ", "
              _
              "Operation: new = old" _ operation:operation()
              _
              "Test: divisible by" _ test_divisible_by:number()
              _
              "If true: throw to monkey" _ true_target:number()
              _
              "If false: throw to monkey" _ false_target:number()

              {
                Monkey {
                    items: items.into(),
                    operation,
                    test_divisible_by,
                    true_target,
                    false_target,
                    inspected_count: 0,
                }
              }

        pub rule monkeys() -> Vec<Monkey>
              = monkeys:monkey() ++ _ { monkeys }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut monkeys = monkeys::monkeys(input.trim())?;

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                let monkey = monkeys
                    .get_mut(i)
                    .ok_or(AdventOfCodeError::InsufficientMonkeysError(i))?;

                if monkey.items.is_empty() {
                    break;
                }

                if let Some((worry, target)) = monkey.get_target() {
                    let target_monkey = monkeys
                        .get_mut(target)
                        .ok_or(AdventOfCodeError::InsufficientMonkeysError(target))?;

                    target_monkey.items.push_back(worry);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));

    let monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.inspected_count)
        .take(2)
        .product();

    Ok(monkey_business)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut monkeys = monkeys::monkeys(input.trim())?;

    let least_common_multiple: usize = monkeys.iter().map(|m| m.test_divisible_by).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            loop {
                let monkey = monkeys.get_mut(i)
                    .ok_or(AdventOfCodeError::InsufficientMonkeysError(i))?;

                if monkey.items.is_empty() {
                    break;
                }

                if let Some((worry, target)) = monkey.get_worrisome_target(least_common_multiple) {
                    let target_monkey = monkeys.get_mut(target)
                        .ok_or(AdventOfCodeError::InsufficientMonkeysError(target))?;

                    target_monkey.items.push_back(worry);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));

    let monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.inspected_count)
        .take(2)
        .product();

    Ok(monkey_business)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(10_605), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2_713_310_158), get_part_two(INPUT));
    }
}
