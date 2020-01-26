use std::fs;

use regex::Regex;
use serde_json::Value;

fn get_sum(input: &str) -> isize {
    let numbers = Regex::new(r#"(-?\d+)\b"#).unwrap();

    numbers
        .captures_iter(input)
        .map(|captures| captures[1].parse::<isize>().unwrap())
        .sum()
}

fn parse_value(value: &Value) -> isize {
    match value {
        number if value.is_i64() => number.as_i64().unwrap() as isize,
        array if array.is_array() => {
            let array = array.as_array().unwrap();
            array.iter().map(|child| parse_value(child)).sum()
        }
        object if object.is_object() => {
            let object = object.as_object().unwrap();
            for (_, v) in object.iter() {
                if v.is_string() && v.as_str().unwrap() == "red" {
                    return 0;
                }
            }
            object.iter().map(|(_, v)| parse_value(v)).sum()
        }
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of all numbers in the document? {}",
        get_sum(&input),
    );

    let value = serde_json::from_str(&input).unwrap();
    println!(
        r#"Ignore any object…which has any property with the value "red"… {}"#,
        parse_value(&value),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(6, get_sum("[1,2,3]"));
        assert_eq!(6, get_sum(r#"{"a":2,"b":4}"#));
        assert_eq!(6, parse_value(&serde_json::from_str("[1,2,3]").unwrap()));
        assert_eq!(
            6,
            parse_value(&serde_json::from_str(r#"{"a":2,"b":4}"#).unwrap())
        );
        assert_eq!(
            6,
            parse_value(&serde_json::from_str(r#"[1,"red",5]"#).unwrap())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(3, get_sum("[[[3]]]"));
        assert_eq!(3, get_sum(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(3, parse_value(&serde_json::from_str("[[[3]]]").unwrap()));
        assert_eq!(
            3,
            parse_value(&serde_json::from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            4,
            parse_value(&serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#).unwrap())
        );
    }

    #[test]
    fn test_0() {
        assert_eq!(0, get_sum(r#"{"a":[-1,1]}"#));
        assert_eq!(0, get_sum(r#"[-1,{"a":1}]"#));
        assert_eq!(0, get_sum("{}"));
        assert_eq!(0, get_sum("[]"));
        assert_eq!(
            0,
            parse_value(&serde_json::from_str(r#"{"a":[-1,1]}"#).unwrap())
        );
        assert_eq!(
            0,
            parse_value(&serde_json::from_str(r#"[-1,{"a":1}]"#).unwrap())
        );
        assert_eq!(0, parse_value(&serde_json::from_str("{}").unwrap()));
        assert_eq!(0, parse_value(&serde_json::from_str("[]").unwrap()));
        assert_eq!(
            0,
            parse_value(&serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap())
        );
    }
}
