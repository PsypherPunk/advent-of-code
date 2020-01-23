use std::fs;

fn get_raw_decoded_difference(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.chars().count() - get_decoded_count(line))
        .sum()
}

fn get_encoded_raw_difference(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| get_encoded_count(line) - line.chars().count())
        .sum()
}

/// Parse characters in turn, validating subsequent characters.
///
/// There doesn't appear to be anything corresponding to Python's
/// `eval()` to parse the string via the standard library.
fn get_decoded_count(input: &str) -> usize {
    let mut count = 0;
    let mut chars = input.chars().peekable();

    while chars.peek().is_some() {
        let c = chars.next().unwrap();

        match c {
            '"' => continue,
            '\\' => match chars.next().unwrap() {
                '\\' | '"' => {}
                'x' => {
                    if !chars.next().unwrap().is_ascii_hexdigit()
                        || !chars.next().unwrap().is_ascii_hexdigit()
                    {
                        panic!("Invalid escape sequence!")
                    }
                }
                _ => panic!("Invalid escape sequence!"),
            },
            _ => {}
        }
        count += 1;
    }
    count
}

fn get_encoded_count(input: &str) -> usize {
    let mut encoded = String::from("\"");
    for c in input.chars() {
        match c {
            '"' => encoded.push_str(r#"\""#),
            '\\' => encoded.push_str(r#"\\"#),
            c => encoded.push(c),
        }
    }
    encoded.push('"');

    encoded.chars().count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the number of characters of code for string literals minus the number of characters in memory…? {}",
        get_raw_decoded_difference(&input),
    );

    println!(
        "…the total number of characters to represent the newly encoded strings minus the number of characters of code in each original string literal… {}",
        get_encoded_raw_difference(&input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoded_difference() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(12, get_raw_decoded_difference(&input));
    }

    #[test]
    fn test_encoding() {
        let input = r#""""#;

        assert_eq!(2, input.chars().count());
        assert_eq!(6, get_encoded_count(&input));
    }

    #[test]
    fn test_encoded_difference() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        assert_eq!(19, get_encoded_raw_difference(&input));
    }
}
