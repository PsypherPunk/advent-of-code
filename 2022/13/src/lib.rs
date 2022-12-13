use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
enum Value {
    Integer(usize),
    List(Vec<Value>),
}

pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            let left: Value = serde_json::from_str(left).unwrap();
            let right: Value = serde_json::from_str(right).unwrap();

            is_ordered(&left, &right)
        })
        .enumerate()
        .filter(|(_, result)| matches!(*result, Some(true)))
        .map(|(index, _)| index + 1)
        .sum()
}

fn is_ordered(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        // "If both values are integers, the lower integer should come first."
        (Value::Integer(left), Value::Integer(right)) => match left.cmp(right) {
            Ordering::Less => Some(true),
            Ordering::Equal => None,
            Ordering::Greater => Some(false),
        },
        // "If both values are lists, compare the first value of each list, then…"
        (Value::List(left), Value::List(right)) => {
            let ordered = left
                .iter()
                .zip(right)
                .map(|(left, right)| is_ordered(left, right))
                .find(|is_ordered| is_ordered.is_some());

            match ordered {
                Some(o) => o,
                None => match left.len().cmp(&right.len()) {
                    Ordering::Less => Some(true),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(false),
                },
            }
        }
        (left @ Value::List(_), _right @ Value::Integer(r)) => {
            is_ordered(left, &Value::List(vec![Value::Integer(*r)]))
        }
        (_left @ Value::Integer(l), right @ Value::List(_)) => {
            is_ordered(&Value::List(vec![Value::Integer(*l)]), right)
        }
    }
}

pub fn get_part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(13, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
