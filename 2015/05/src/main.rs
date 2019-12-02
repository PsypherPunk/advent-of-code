use std::fs::File;
use std::io::prelude::*;

use fancy_regex::Regex as FRegex;
use regex::Regex;

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn is_string_nice(string: &str) -> bool {
    let vowels = Regex::new(r"[aeiou]").unwrap();
    let disallowed = FRegex::new(r"(ab|cd|pq|xy)").unwrap();
    let twice_in_a_row = FRegex::new(r"(.)\1").unwrap();

    vowels.find_iter(string).map(|m| m.as_str()).collect::<Vec<&str>>().len() >= 3
        && disallowed.find(string).unwrap().is_none()
        && twice_in_a_row.find(string).unwrap().is_some()
}

fn is_string_nice_new_rules(string: &str) -> bool {
    let two_letters_repeated= FRegex::new(r"(..).*\1").unwrap();
    let one_letter_repeated_with_one_between = FRegex::new(r"(.).\1").unwrap();

    two_letters_repeated.find(string).unwrap().is_some()
        && one_letter_repeated_with_one_between.find(string).unwrap().is_some()
}

fn count_nice_strings(input: &String) -> usize {
    input
        .lines()
        .map(|line| is_string_nice(line))
        .filter(|nn| *nn)
        .count()
}

fn count_nice_strings_new_rules(input: String) -> usize {
    input
        .lines()
        .map(|line| is_string_nice_new_rules(line))
        .filter(|nn| *nn)
        .count()
}

fn main() {
    let input = read_input();
    println!("Number of nice strings: {}", count_nice_strings(&input));
    println!("Number of nice strings under the new rules: {}", count_nice_strings_new_rules(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ugknbfddgicrmopn() {
        assert_eq!(is_string_nice("ugknbfddgicrmopn"), true);
    }

    #[test]
    fn test_aaa() {
        assert_eq!(is_string_nice("aaa"), true);
    }

    #[test]
    fn test_jchzalrnumimnmhp() {
        assert_eq!(is_string_nice("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn test_haegwjzuvuyypxyu() {
        assert_eq!(is_string_nice("haegwjzuvuyypxyu"), false);
    }

    #[test]
    fn test_dvszwmarrgswjxmb() {
        assert_eq!(is_string_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_qjhvhtzxzqqjkmpb() {
        assert_eq!(is_string_nice_new_rules("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn test_xxyxx() {
        assert_eq!(is_string_nice_new_rules("xxyxx"), true);
    }

    #[test]
    fn test_uurcxstgmygtbstg() {
        assert_eq!(is_string_nice_new_rules("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn test_ieodomkazucvgmuy() {
        assert_eq!(is_string_nice_new_rules("ieodomkazucvgmuy"), false);
    }
}
