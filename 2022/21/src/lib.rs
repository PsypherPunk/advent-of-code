use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operation<'a> {
    Value(usize),
    Plus((&'a str, &'a str)),
    Minus((&'a str, &'a str)),
    Multiply((&'a str, &'a str)),
    Divide((&'a str, &'a str)),
}

peg::parser! {
    pub grammar monkeys() for str {
        rule _() = [' ' | '\n']*

        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule monkey() -> &'input str
            = m:$(['a'..='z']*<4>)
                { m }

        rule maths() -> Operation<'input>
            = a:monkey() _ op:$("+" / "-" / "*" / "/") _ b:monkey()
                {
                    match op {
                        "+" => Operation::Plus((a, b)),
                        "-" => Operation::Minus((a, b)),
                        "*" => Operation::Multiply((a, b)),
                        "/" => Operation::Divide((a, b)),
                        _ => unreachable!(),
                    }
                }

        rule operation() -> Operation<'input>
            = value:number() { Operation::Value(value) } / op:maths() { op }

        rule riddle_step() -> (&'input str, Operation<'input>)
            = monkey:monkey() ":" _ operation:operation()
                { (monkey, operation) }

        pub rule riddle() -> HashMap<&'input str, Operation<'input>>
            = steps:riddle_step() ++ _
                { steps.into_iter().collect() }

    }
}

fn solve_riddle(monkey: &str, steps: &HashMap<&str, Operation>) -> usize {
    let operation = steps.get(monkey).unwrap();

    match operation {
        Operation::Value(number) => *number,
        Operation::Plus((a, b)) => solve_riddle(a, steps) + solve_riddle(b, steps),
        Operation::Minus((a, b)) => solve_riddle(a, steps).saturating_sub(solve_riddle(b, steps)),
        Operation::Multiply((a, b)) => solve_riddle(a, steps) * solve_riddle(b, steps),
        Operation::Divide((a, b)) => solve_riddle(a, steps) / solve_riddle(b, steps),
    }
}

fn maybe_solve_riddle(monkey: &str, steps: &HashMap<&str, Operation>) -> Option<usize> {
    if monkey == "humn" {
        return None;
    }

    let operation = steps.get(monkey).unwrap();

    match operation {
        Operation::Value(number) => Some(*number),
        Operation::Plus((a, b)) => {
            Some(maybe_solve_riddle(a, steps)? + maybe_solve_riddle(b, steps)?)
        }
        Operation::Minus((a, b)) => {
            Some(maybe_solve_riddle(a, steps)?.saturating_sub(maybe_solve_riddle(b, steps)?))
        }
        Operation::Multiply((a, b)) => {
            Some(maybe_solve_riddle(a, steps)? * maybe_solve_riddle(b, steps)?)
        }
        Operation::Divide((a, b)) => {
            Some(maybe_solve_riddle(a, steps)? / maybe_solve_riddle(b, steps)?)
        }
    }
}

fn reverse_riddle(monkey: &str, steps: &HashMap<&str, Operation>, target: usize) -> usize {
    match monkey {
        "humn" => target,
        monkey => match steps.get(monkey).unwrap() {
            Operation::Value(number) => *number,
            Operation::Plus((a, b)) => match maybe_solve_riddle(a, steps) {
                None => reverse_riddle(a, steps, target - solve_riddle(b, steps)),
                Some(_) => reverse_riddle(b, steps, target - solve_riddle(a, steps)),
            },
            Operation::Minus((a, b)) => match maybe_solve_riddle(a, steps) {
                None => reverse_riddle(a, steps, target + solve_riddle(b, steps)),
                Some(_) => reverse_riddle(b, steps, solve_riddle(a, steps) - target),
            },
            Operation::Multiply((a, b)) => match maybe_solve_riddle(a, steps) {
                None => reverse_riddle(a, steps, target / solve_riddle(b, steps)),
                Some(_) => reverse_riddle(b, steps, target / solve_riddle(a, steps)),
            },
            Operation::Divide((a, b)) => match maybe_solve_riddle(a, steps) {
                None => reverse_riddle(a, steps, target * solve_riddle(b, steps)),
                Some(_) => reverse_riddle(b, steps, solve_riddle(a, steps) * target),
            },
        },
    }
}

pub fn get_part_one(input: &str) -> usize {
    let riddle = monkeys::riddle(input.trim()).unwrap();

    solve_riddle("root", &riddle)
}

pub fn get_part_two(input: &str) -> usize {
    let riddle = monkeys::riddle(input.trim()).unwrap();

    let ((a_monkey, a_answer), (b_monkey, b_answer)) = match riddle.get("root") {
        Some(Operation::Plus((a, b))) => (
            (a, maybe_solve_riddle(a, &riddle)),
            (b, maybe_solve_riddle(b, &riddle)),
        ),
        _ => unreachable!(),
    };

    match (a_answer, b_answer) {
        (Some(a_answer), None) => reverse_riddle(b_monkey, &riddle, a_answer),
        (None, Some(b_answer)) => reverse_riddle(a_monkey, &riddle, b_answer),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(152, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(301, get_part_two(INPUT));
    }
}
