use std::collections::HashMap;
use std::str::FromStr;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Rule {
    lower: usize,
    upper: usize,
}

pub struct Document {
    rules: HashMap<String, Vec<Rule>>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [lower, upper] = match s.trim().split('-').collect::<Vec<&str>>()[..] {
            [a, b] => [a, b],
            _ => return Err(()),
        };
        Ok(Rule {
            lower: lower.parse().unwrap(),
            upper: upper.parse().unwrap(),
        })
    }
}

impl FromStr for Document {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [rules, your, nearby] = match s.trim().split("\n\n").collect::<Vec<&str>>()[..] {
            [a, b, c] => [a, b, c],
            _ => return Err(()),
        };

        let rules = rules
            .trim()
            .lines()
            .map(|line| {
                let [field, rules] = match line.split(':').collect::<Vec<&str>>()[..] {
                    [a, b] => [a, b],
                    _ => panic!(),
                };
                let field = field.split_whitespace().next().unwrap();
                let rules = rules
                    .trim()
                    .split(" or ")
                    .map(|rule| Rule::from_str(rule).unwrap())
                    .collect();

                (field.to_string(), rules)
            })
            .collect();

        let your_ticket = your
            .trim()
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();

        let nearby_tickets = nearby
            .trim()
            .lines()
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|number| number.parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(Document {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

impl Document {
    pub fn get_ticket_scanning_error_rate(&self) -> usize {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .filter(|&value| {
                        self.rules
                            .values()
                            .map(|rules| {
                                rules
                                    .iter()
                                    .find(|rule| *value >= rule.lower && *value <= rule.upper)
                                    .is_none()
                            })
                            .all(|matches_rule| matches_rule)
                    })
                    .collect::<Vec<_>>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

        let document = Document::from_str(&input).unwrap();

        assert_eq!(71, document.get_ticket_scanning_error_rate());
    }
}
