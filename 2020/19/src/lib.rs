use std::collections::HashMap;
use std::str::FromStr;

enum Rule {
    SingleCharacter(char),
    Rule(usize),
    AtLeastOne(Box<Rule>, Box<Rule>),
    MatchPair(Box<Rule>, Box<Rule>),
    MatchTrio(Box<Rule>, Box<Rule>, Box<Rule>),
}

pub struct SatelliteConnection {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl FromStr for SatelliteConnection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, messages) = match s.trim().split("\n\n").collect::<Vec<_>>()[..] {
            [a, b] => (a, b),
            _ => panic!("Invalid input format."),
        };

        let rules = rules
            .trim()
            .lines()
            .map(|line| {
                let (id, rule) = match line.split(':').collect::<Vec<_>>()[..] {
                    [a, b] => (
                        a.trim().parse::<usize>().unwrap(),
                        Rule::from_str(b.trim()).unwrap(),
                    ),
                    _ => panic!("Invalid line: {}", line),
                };

                (id, rule)
            })
            .collect::<HashMap<_, _>>();

        let messages = messages
            .trim()
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(Self { rules, messages })
    }
}

impl SatelliteConnection {
    pub fn get_valid_message_count(&self) -> i64 {
        self.messages
            .iter()
            .map(|line| {
                let message = line.chars().collect::<Vec<_>>();
                match self
                    .rules
                    .get(&0)
                    .unwrap()
                    .matches(&self.rules, &message)
                    .into_iter()
                    .find(|match_| match_.is_empty())
                {
                    Some(_) => 1,
                    None => 0,
                }
            })
            .sum()
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.contains('|') => {
                let (sub_rule_a, sub_rule_b) = match s.split('|').collect::<Vec<_>>()[..] {
                    [a, b] => (a, b),
                    _ => panic!("Invalid rule: {}", s),
                };

                Ok(Rule::AtLeastOne(
                    Box::new(Rule::from_str(sub_rule_a.trim()).unwrap()),
                    Box::new(Rule::from_str(sub_rule_b.trim()).unwrap()),
                ))
            }
            s if s.starts_with('"') => Ok(Rule::SingleCharacter(s.chars().nth(1).unwrap())),
            s if s.contains(' ') => match s.split_whitespace().collect::<Vec<_>>()[..] {
                [sub_rule_a, sub_rule_b, sub_rule_c] => Ok(Rule::MatchTrio(
                    Box::new(Rule::from_str(sub_rule_a).unwrap()),
                    Box::new(Rule::from_str(sub_rule_b).unwrap()),
                    Box::new(Rule::from_str(sub_rule_c).unwrap()),
                )),
                [sub_rule_a, sub_rule_b] => Ok(Rule::MatchPair(
                    Box::new(Rule::from_str(sub_rule_a).unwrap()),
                    Box::new(Rule::from_str(sub_rule_b).unwrap()),
                )),
                _ => Err(()),
            },
            _ => match s.parse() {
                Ok(n) => Ok(Rule::Rule(n)),
                Err(_) => Err(()),
            },
        }
    }
}

impl Rule {
    fn matches<'a>(&self, rules: &'a HashMap<usize, Rule>, tail: &'a [char]) -> Vec<&'a [char]> {
        if tail.is_empty() {
            return Vec::new();
        }
        match self {
            Rule::Rule(n) => rules.get(n).unwrap().matches(rules, tail),
            Rule::AtLeastOne(sub_rule_a, sub_rule_b) => sub_rule_a
                .matches(rules, tail)
                .into_iter()
                .chain(sub_rule_b.matches(rules, tail))
                .collect(),
            Rule::SingleCharacter(c) => match tail.starts_with(&[*c]) {
                true => vec![&tail[1..]],
                _ => vec![],
            },
            Rule::MatchPair(sub_rule_a, sub_rule_b) => sub_rule_a
                .matches(rules, tail)
                .iter()
                .flat_map(|match_| sub_rule_b.matches(rules, match_))
                .collect(),
            Rule::MatchTrio(sub_rule_a, sub_rule_b, sub_rule_c) => sub_rule_a
                .matches(rules, tail)
                .iter()
                .flat_map(|match_| {
                    sub_rule_b
                        .matches(rules, match_)
                        .iter()
                        .flat_map(|match_| sub_rule_c.matches(rules, match_))
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb;"#;

        let satellite = SatelliteConnection::from_str(&input).unwrap();

        assert_eq!(2, satellite.get_valid_message_count());
    }
}
