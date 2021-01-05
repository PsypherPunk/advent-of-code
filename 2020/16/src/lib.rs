use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

type Ticket = Vec<usize>;

pub struct Document {
    rules: HashMap<String, Vec<Rule>>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

struct Rule(RangeInclusive<usize>);

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [lower, upper] = match s.trim().split('-').collect::<Vec<&str>>()[..] {
            [a, b] => [a, b],
            _ => return Err(()),
        };
        Ok(Rule(RangeInclusive::new(
            lower.parse().unwrap(),
            upper.parse().unwrap(),
        )))
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
    fn get_invalid_ticket_values(&self, ticket: &[usize]) -> Vec<usize> {
        ticket
            .iter()
            .filter(|&value| {
                self.rules
                    .values()
                    .map(|rules| rules.iter().find(|rule| rule.0.contains(value)).is_none())
                    .all(|matches_rule| matches_rule)
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    fn is_valid_ticket(&self, ticket: &[usize]) -> bool {
        self.get_invalid_ticket_values(&ticket).is_empty()
    }

    pub fn get_ticket_scanning_error_rate(&self) -> usize {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| self.get_invalid_ticket_values(&ticket))
            .sum()
    }

    fn get_your_ticket(&self) -> HashMap<String, usize> {
        let valid_tickets = self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.is_valid_ticket(&ticket))
            .collect::<Vec<_>>();

        let columnar = (0..self.your_ticket.len())
            .map(|index| {
                valid_tickets
                    .iter()
                    .map(|ticket| ticket[index])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut rule_indices = self
            .rules
            .iter()
            .map(|(field, rules)| {
                (
                    field,
                    columnar
                        .iter()
                        .enumerate()
                        .filter(|(_, column)| {
                            column
                                .iter()
                                .all(|value| rules.iter().any(|rule| rule.0.contains(value)))
                        })
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        rule_indices.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));
        let mut resolved_indices = HashSet::new();

        rule_indices
            .iter()
            .map(|(field, indices)| {
                match indices
                    .iter()
                    .find(|&index| !resolved_indices.contains(index))
                {
                    Some(index) => {
                        resolved_indices.insert(*index);
                        (field.to_string(), *self.your_ticket.get(*index).unwrap())
                    }
                    None => unreachable!(),
                }
            })
            .collect()
    }

    pub fn get_departure_product(&self) -> usize {
        self.get_your_ticket()
            .iter()
            .filter(|(field, _)| field.starts_with("departure"))
            .map(|(_, value)| value)
            .product()
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

    #[test]
    fn test_part_two() {
        let input = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

        let document = Document::from_str(&input).unwrap();
        let your_ticket = document.get_your_ticket();

        assert_eq!(11, *your_ticket.get("row").unwrap());
        assert_eq!(12, *your_ticket.get("class").unwrap());
        assert_eq!(13, *your_ticket.get("seat").unwrap());
    }
}
