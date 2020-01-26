use std::fs;

use regex::Regex;

fn get_sum(input: &str) -> isize {
    let numbers = Regex::new(r#"(-?\d+)\b"#).unwrap();

    numbers
        .captures_iter(input)
        .map(|captures| captures[1].parse::<isize>().unwrap())
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the sum of all numbers in the document? {}",
        get_sum(&input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(6, get_sum("[1,2,3]"));
        assert_eq!(6, get_sum(r#"{"a":2,"b":4}"#));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, get_sum("[[[3]]]"));
        assert_eq!(3, get_sum(r#"{"a":{"b":4},"c":-1}"#));
    }

    #[test]
    fn test_0() {
        assert_eq!(0, get_sum(r#"{"a":[-1,1]}"#));
        assert_eq!(0, get_sum(r#"[-1,{"a":1}]"#));
        assert_eq!(0, get_sum("{}"));
        assert_eq!(0, get_sum("[]"));
    }
}
