peg::parser! {
    grammar parser() for str {
        rule number() -> usize = _ n:$(['0'..='9']) _ { n.parse().unwrap() }

        pub rule expression() -> usize = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }

            n:number() { n }

            _ "(" e:expression() ")" _ { e }
        }

        rule _() = [' ']*
    }
}

fn get_homework_line_result(line: &str) -> usize {
    parser::expression(line).unwrap()
}

pub fn get_homework_sum(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| get_homework_line_result(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(26, get_homework_line_result("2 * 3 + (4 * 5)"));
        assert_eq!(437, get_homework_line_result("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            12240,
            get_homework_line_result("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        );
        assert_eq!(
            13632,
            get_homework_line_result("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
