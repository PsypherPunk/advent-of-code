pub fn get_score_for_input(input: &str) -> usize {
    let mut chars = input.chars().peekable();

    let mut depth = 0;
    let mut score = 0;
    let mut garbage = false;

    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        match c {
            '{' => {
                if !garbage {
                    depth += 1;
                }
            }
            '}' => {
                if !garbage {
                    score += depth;
                    depth -= 1;
                }
            }
            '!' => {
                let _ = chars.next().unwrap();
            }
            '<' => {
                garbage = true;
            }
            '>' => {
                garbage = false;
            }
            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(get_score_for_input("{}"), 1);
        assert_eq!(get_score_for_input("{{{}}}"), 6);
        assert_eq!(get_score_for_input("{{},{}}"), 5);
        assert_eq!(get_score_for_input("{{{},{},{{}}}}"), 16);
        assert_eq!(get_score_for_input("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(get_score_for_input("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(get_score_for_input("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(get_score_for_input("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
