use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Rule<'a> {
    category: usize,
    operation: Operation,
    value: usize,
    target: &'a str,
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    otherwise: &'a str,
}

type Part = Vec<usize>;

impl Operation {
    fn matches(&self, left: usize, right: usize) -> bool {
        match self {
            Operation::LessThan => left < right,
            Operation::GreaterThan => left > right,
        }
    }
}

impl<'a> Workflow<'a> {
    fn perform(&'a self, part: &Part) -> &'a str {
        match self
            .rules
            .iter()
            .find(|rule| rule.operation.matches(part[rule.category], rule.value))
        {
            Some(rule) => rule.target,
            None => self.otherwise,
        }
    }
}

peg::parser! {
    pub grammar ratings() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule label() -> &'input str
            = l:$(['a'..='z' | 'A'..='Z']+)

        rule category() -> usize
            = category:$("x" / "m" / "a" / "s")
            {
                match category {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => unreachable!(),
                }
            }

        rule operation() -> Operation
            = operation:$(">" / "<")
            {
                match operation {
                    ">" => Operation::GreaterThan,
                    "<" => Operation::LessThan,
                    _ => unreachable!(),
                }
            }

        rule rule_() -> Rule<'input>
            = category:category()
              operation:operation()
              value:integer()
              ":"
              target:label()
            {
                Rule {
                    category,
                    operation,
                    value,
                    target,
                }
            }

        pub rule workflow() -> (&'input str, Workflow<'input>)
            = name:label()
              "{"
              rules:rule_() ++ ","
              ","
              otherwise:label()
              "}"
            {
                (
                    name,
                    Workflow {
                        rules,
                        otherwise,
                    },
                )
            }

        rule rating() -> usize
            = label()
              "="
              value:integer()
              {
                value
              }

        rule part() -> Part
            = "{"
              values:rating() ++ ","
              "}"
              {
                values
              }

        pub rule input() -> (HashMap<&'input str, Workflow<'input>>, Vec<Part>)
              = workflows:workflow() ++ _
                _
                parts:part() ++ _
                {
                    (workflows.into_iter().collect(), parts)
                }

    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let (workflows, parts) = ratings::input(input.trim()).map_err(|e| e.to_string())?;

    let ratings = parts
        .iter()
        .filter(|part| {
            // TODO: there must be a nice way to do this…?!
            let mut label = "in";
            while label != "A" && label != "R" {
                label = workflows.get(&label).unwrap().perform(part);
            }
            label == "A"
        })
        .map(|part| part.iter().sum::<usize>())
        .sum();

    Ok(ratings)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    #[test]
    fn test_parse_workflow() {
        let (label, workflow) = ratings::workflow("ex{x>10:one,m<20:two,a>30:R,A}").unwrap();

        assert_eq!("ex", label);
        assert_eq!(3, workflow.rules.len());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(19114), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
