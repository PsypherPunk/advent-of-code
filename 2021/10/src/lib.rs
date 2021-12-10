use std::collections::VecDeque;

pub fn get_part_one(input: &str) -> usize {
    let mut score = 0;

    input.trim().lines().for_each(|line| {
        let mut stack = VecDeque::new();
        line.chars().for_each(|c| match c {
            '(' => stack.push_front(')'),
            '[' => stack.push_front(']'),
            '{' => stack.push_front('}'),
            '<' => stack.push_front('>'),
            closing => {
                let expected = stack.pop_front().unwrap();
                score += match closing == expected {
                    false => match closing {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!(),
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
        assert_eq!(1, 2)
    }
}
