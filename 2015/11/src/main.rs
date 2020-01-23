use itertools::Itertools;

fn has_increasing_straight(password: &str) -> bool {
    password
        .chars()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| b as u8 == (a as u8 + 1) && c as u8 == (b as u8 + 1))
        .any(|b| b)
}

fn has_iol(password: &str) -> bool {
    password
        .chars()
        .filter(|c| vec!['i', 'o', 'l'].contains(c))
        .count()
        > 0
}

fn has_at_least_two_pairs(password: &str) -> bool {
    password
        .chars()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, r)| r.collect::<String>())
        .filter(|group| group.len() == 2)
        .count()
        >= 2
}

fn increment_password(password: &str) -> String {
    let mut next_password = password.chars().rev().collect::<Vec<char>>();
    for i in 0..password.len() {
        match next_password[i] {
            'z' => {
                next_password[i] = 'a';
            }
            _ => {
                next_password[i] = char::from(next_password[i] as u8 + 1);
                break;
            }
        }
    }

    next_password.iter().rev().collect()
}

fn get_next_password(password: &str) -> String {
    let mut password = String::from(password);
    loop {
        if has_increasing_straight(password.as_str())
            && !has_iol(password.as_str())
            && has_at_least_two_pairs(password.as_str())
        {
            break;
        }
        password = increment_password(password.as_str());
    }

    password
}

fn main() {
    let input = "hepxcrrq";

    println!(
        "…what should his next password be? {}",
        get_next_password(input),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hijklmmn() {
        let password = "hijklmmn";

        assert!(has_increasing_straight(password));
        assert!(has_iol(password));
    }

    #[test]
    fn test_abbceffg() {
        let password = "abbceffg";

        assert!(has_at_least_two_pairs(password));
    }

    #[test]
    fn test_abbcegjk() {
        let password = "abbcegjk";

        assert!(!has_at_least_two_pairs(password));
    }

    #[test]
    fn test_abcdefgh() {
        let password = "abcdefgh";

        assert_eq!("abcdffaa", get_next_password(password));
    }

    #[test]
    fn test_ghijklmn() {
        let password = "ghijklmn";

        assert_eq!("ghjaabcc", get_next_password(password));
    }
}
