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

pub fn get_part_two(input: &str) -> Result<usize, String> {
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
                closing => match stack.pop_front() {
                    Some(expected) => closing != expected,
                    None => true,
                },
            })
        })
        .map(|line| {
            let mut stack = VecDeque::new();
            line.chars().try_for_each(|c| {
                match c {
                    '(' | '[' | '{' | '<' => stack.push_front(get_closing_bracket(c)),
                    _ => {
                        stack.pop_front().ok_or_else(|| "".to_owned())?;
                    }
                };
                Ok::<(), String>(())
            })?;
            Ok(stack.iter().fold(0, |acc, c| {
                (5 * acc)
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;

    scores.sort_unstable();

    Ok(scores[scores.len() / 2])
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut score = 0;

    input.trim().lines().try_for_each(|line| {
        let mut stack = VecDeque::new();
        line.chars().try_for_each(|c| {
            match c {
                '(' | '[' | '{' | '<' => stack.push_front(get_closing_bracket(c)),
                closing => {
                    let expected = stack.pop_front().ok_or_else(|| "".to_owned())?;
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
            };
            Ok::<(), String>(())
        })?;
        Ok::<(), String>(())
    })?;

    Ok(score)
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
        assert_eq!(Ok(26397), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(288957), get_part_two(INPUT));
    }
}
