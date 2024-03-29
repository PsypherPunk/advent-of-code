#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::cmp::Ordering;
use std::collections::HashSet;

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidPacketsError(ParseError<LineCol>),
}

impl From<ParseError<LineCol>> for AdventOfCodeError {
    fn from(error: ParseError<LineCol>) -> Self {
        AdventOfCodeError::InvalidPacketsError(error)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    Integer(usize),
    List(Vec<Value>),
}

peg::parser! {
    pub grammar packets() for str {

        rule _() = [' ' | '\n']*

        rule value() -> Value
            = n:number() { Value::Integer(n) } / a:array() { a }

        rule array() -> Value
            = "[" a:value() ** "," "]"
                { Value::List(a) }

        rule number() -> usize
            = n:$(['0'] / ['1'..='9']['0'..='9']*)
                {? n.parse().or(Err("int()"))}

        pub rule packets() -> Vec<Value>
            = _ p:array() ++ _
                { p }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let packets = packets::packets(input.trim())?;

    let sum = packets
        .chunks_exact(2)
        .map(|pair| is_ordered(&pair[0], &pair[1]))
        .enumerate()
        .filter(|(_, result)| matches!(*result, Some(true)))
        .map(|(index, _)| index + 1)
        .sum();

    Ok(sum)
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
        // "If exactly one value is an integer, convert the integer to a list…"
        (left @ Value::List(_), _right @ Value::Integer(r)) => {
            is_ordered(left, &Value::List(vec![Value::Integer(*r)]))
        }
        (_left @ Value::Integer(l), right @ Value::List(_)) => {
            is_ordered(&Value::List(vec![Value::Integer(*l)]), right)
        }
    }
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let mut packets = packets::packets(input.trim())?;

    let divider_packets = packets::packets("[[2]]\n[[6]]")?
        .into_iter()
        .collect::<HashSet<_>>();

    packets.extend(divider_packets.clone());

    packets.sort_by(|left, right| match is_ordered(left, right) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let product = packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| match divider_packets.contains(packet) {
            true => Some(index + 1),
            false => None,
        })
        .product();

    Ok(product)
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
        assert_eq!(Ok(13), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(140), get_part_two(INPUT));
    }

    #[test]
    fn test_parser() {
        let packets = packets::packets("[[1],[2,3,4]]").unwrap();

        let expected = vec![Value::List(vec![
            Value::List(vec![Value::Integer(1)]),
            Value::List(vec![
                Value::Integer(2),
                Value::Integer(3),
                Value::Integer(4),
            ]),
        ])];

        assert_eq!(packets, expected);
    }
}
