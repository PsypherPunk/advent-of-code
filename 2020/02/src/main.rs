use std::fs;

struct Policy {
    min_count: usize,
    max_count: usize,
    letter: char,
}

fn get_policies_passwords(input: &str) -> Vec<(Policy, &str)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let counts = parts[0].split('-').collect::<Vec<&str>>();
            let letter = parts[1].chars().next().unwrap();
            (
                Policy {
                    min_count: counts[0].parse().unwrap(),
                    max_count: counts[1].parse().unwrap(),
                    letter,
                },
                parts[2],
            )
        })
        .collect()
}

fn is_valid_password(policy: &Policy, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == policy.letter).count();

    count >= policy.min_count && count <= policy.max_count
}

fn get_valid_password_count(input: &str) -> usize {
    let policies_passwords = get_policies_passwords(&input);

    policies_passwords
        .iter()
        .filter(|(policy, password)| is_valid_password(policy, password))
        .count()
}

fn is_valid_otcas_password(policy: &Policy, password: &str) -> bool {
    let a = password.chars().nth(policy.min_count - 1).unwrap() == policy.letter;
    let b = password.chars().nth(policy.max_count - 1).unwrap() == policy.letter;

    a ^ b
}

fn get_valid_otcas_password_count(input: &str) -> usize {
    let policies_passwords = get_policies_passwords(&input);

    policies_passwords
        .iter()
        .filter(|(policy, password)| is_valid_otcas_password(policy, password))
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many passwords are valid…? {}",
        get_valid_password_count(&input),
    );

    println!(
        "How many passwords are valid…? {}",
        get_valid_otcas_password_count(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc""#;

        assert_eq!(2, get_valid_password_count(&input));
    }

    #[test]
    fn test_part_two() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc""#;

        assert_eq!(1, get_valid_otcas_password_count(&input));
    }
}
