use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {}

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
        if self.items.is_empty() {
            return None;
        }

        let worry = self.items.pop_front().unwrap();
        self.inspected_count += 1;
        println!("  Monkey inspects an item with a worry level of {}.", worry);

        let worry = match self.operation {
            Operation::Multiply(n) => {
                println!("    Worry level is multiplied by {} to {}.", n, worry * n);
                worry * n
            }
            Operation::Add(n) => {
                println!("    Worry level increases by {} to {}.", n, worry + n);
                worry + n
            }
            Operation::Squared => {
                println!(
                    "    Worry level is multiplied by itself to {}.",
                    worry * worry
                );
                worry * worry
            }
        } / 3;
        println!(
            "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
            worry
        );
        let target = match worry % self.test_divisible_by {
            0 => {
                println!(
                    "    Current worry level is divisible by {}.",
                    self.test_divisible_by
                );
                self.true_target
            }
            _ => {
                println!(
                    "    Current worry level is not divisible by {}.",
                    self.test_divisible_by
                );
                self.false_target
            }
        };

        Some((worry, target))
    }
}

peg::parser! {
    grammar monkeys() for str {
        rule _() = [' ' | '\n']*

        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }

        rule operation() -> Operation
            = s:$("* old" / "*" / "+") _ n:number()? {?
                match s {
                    "* old" => Ok(Operation::Squared),
                    "+" => match n {
                            Some(n) => Ok(Operation::Add(n)),
                            None => Err("usize"),
                        },
                    "*" => match n {
                            Some(n) => Ok(Operation::Multiply(n)),
                            None => Err("usize"),
                        _ => Err("operation"),
                    }
                    _ => Err("operation"),
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

pub fn get_part_one(input: &str) -> usize {
    let mut monkeys = monkeys::monkeys(input.trim()).unwrap();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            println!("Monkey {}:", i);

            loop {
                let monkey = monkeys.get_mut(i).unwrap();
                if monkey.items.is_empty() {
                    break;
                }

                if let Some((worry, target)) = monkey.get_target() {
                    let target_monkey = monkeys.get_mut(target).unwrap();
                    target_monkey.items.push_back(worry);
                    println!(
                        "    Item with worry level {} is thrown to monkey {}.",
                        worry, target
                    );
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_count.cmp(&a.inspected_count));

    monkeys
        .iter()
        .map(|monkey| monkey.inspected_count)
        .take(2)
        .product()
}

pub fn get_part_two(input: &str) -> usize {
    0
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
        assert_eq!(10_605, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
