use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Regulations {
    rules: HashMap<String, Vec<Rule>>,
}

#[derive(Debug)]
struct Rule {
    bag_type: String,
    count: usize,
}

impl Regulations {
    fn get_bags_containing(&self, bag_type: &str) -> Vec<&str> {
        let mut containers = self
            .rules
            .iter()
            .filter(|(_, rules)| {
                rules
                    .iter()
                    .filter(|rule| rule.bag_type == bag_type)
                    .count()
                    > 0
            })
            .map(|(container, _)| container.as_str())
            .collect::<Vec<_>>();

        let contained = containers
            .iter()
            .flat_map(|container| self.get_bags_containing(&container))
            .collect::<Vec<_>>();

        containers.extend(contained);
        containers.sort_unstable();
        containers.dedup();

        containers
    }

    fn get_content_count(&self, bag_type: &str) -> usize {
        let rules = self.rules.get(bag_type).unwrap();

        1 + match rules.len() {
            0 => 0,
            _ => rules
                .iter()
                .map(|rule| rule.count * self.get_content_count(&rule.bag_type))
                .sum(),
        }
    }
}

impl FromStr for Regulations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .trim()
            .lines()
            .map(
                |line| match line.split(" contain ").collect::<Vec<&str>>()[..] {
                    [bag_type, "no other bags."] => {
                        let bag_type = bag_type[..(bag_type.len() - 5)].to_string();
                        (bag_type, vec![])
                    }
                    [bag_type, contents] => {
                        let bag_type = bag_type[..(bag_type.len() - 5)].to_string();
                        let rules = contents
                            .split(", ")
                            .map(|content| Rule::from_str(content).unwrap())
                            .collect::<Vec<_>>();
                        (bag_type, rules)
                    }
                    _ => panic!(),
                },
            )
            .collect::<HashMap<_, _>>();

        Ok(Regulations { rules })
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s.find(' ').unwrap();
        let end = s.find(" bag").unwrap();
        let count = s[..start].parse().unwrap();

        Ok(Rule {
            bag_type: s[(start + 1)..end].to_string(),
            count,
        })
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let regulations = Regulations::from_str(&input).unwrap();

    println!(
        "How many bag colors can eventually contain at least one shiny gold bag? {}",
        regulations.get_bags_containing("shiny gold").len(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

        let regulations = Regulations::from_str(&input).unwrap();

        assert_eq!(12, regulations.get_content_count("vibrant plum"));
        assert_eq!(1, regulations.get_content_count("faded blue"));

        assert_eq!(4, regulations.get_bags_containing("shiny gold").len());
    }
}
