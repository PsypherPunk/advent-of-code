peg::parser! {
    grammar parser() for str {
        rule number() -> usize = _ n:$(['0'..='9']) _ { n.parse().unwrap() }

        pub rule expression() -> usize = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            --
            n:number() { n }

            _ "(" e:expression() ")" _ { e }
        }

        pub rule advanced() -> usize = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            n:number() { n }

            _ "(" e:advanced() ")" _ { e }
        }

        rule _() = [' ']*
    }
}

fn get_homework_line_result(line: &str) -> usize {
    parser::expression(line).unwrap()
}

fn get_advanced_homework_line_result(line: &str) -> usize {
    parser::advanced(line).unwrap()
}

pub fn get_homework_sum(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| get_homework_line_result(line))
        .sum()
}

pub fn get_advanced_homework_sum(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| get_advanced_homework_line_result(line))
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
            get_homework_line_result("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            51,
            get_advanced_homework_line_result("1 + (2 * 3) + (4 * (5 + 6))")
        );
        assert_eq!(46, get_advanced_homework_line_result("2 * 3 + (4 * 5)"));
        assert_eq!(
            1445,
            get_advanced_homework_line_result("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        );
        assert_eq!(
            669060,
            get_advanced_homework_line_result("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        );
        assert_eq!(
            23340,
            get_advanced_homework_line_result("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        );
    }
}
