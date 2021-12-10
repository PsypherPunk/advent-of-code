use std::collections::VecDeque;

/// The ASCII codes for all brackets are 2 away from each other…except
/// for `(` and `)`, natch.
fn get_closing_bracket(c: char) -> char {
    match c {
        '(' => ')',
        '[' | '{' | '<' => char::from(c as u8 + 2),
        _ => unreachable!(),
    }
}

pub fn get_part_two(input: &str) -> usize {
    let mut scores = input
        .trim()
        .lines()
        .filter(|line| {
            let mut stack = VecDeque::new();
            !line.chars().any(|c| match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_front(get_closing_bracket(c));
                    false
                }
                closing => {
                    let expected = stack.pop_front().unwrap();
                    closing != expected
                }
            })
        })
        .map(|line| {
            let mut stack = VecDeque::new();
            line.chars().for_each(|c| match c {
                '(' | '[' | '{' | '<' => stack.push_front(get_closing_bracket(c)),
                _ => {
                    stack.pop_front().unwrap();
                }
            });
            stack.iter().fold(0, |acc, c| {
                (5 * acc)
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

pub fn get_part_one(input: &str) -> usize {
    let mut score = 0;

    input.trim().lines().for_each(|line| {
        let mut stack = VecDeque::new();
        line.chars().for_each(|c| match c {
            '(' | '[' | '{' | '<' => stack.push_front(get_closing_bracket(c)),
            closing => {
                let expected = stack.pop_front().unwrap();
                score += match closing == expected {
                    false => match closing {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    },
                    true => 0,
                }
            }
        });
    });

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(26397, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(288957, get_part_two(INPUT));
    }
}
