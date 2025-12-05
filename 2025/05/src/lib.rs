use std::ops::RangeInclusive;

struct Database {
    ranges: Vec<RangeInclusive<usize>>,
    ids: Vec<usize>,
}

peg::parser! {
    pub grammar database() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule range() -> RangeInclusive<usize>
            = start:integer() "-" end:integer() { start..=end }

        rule ranges() -> Vec<RangeInclusive<usize>>
            = rs:range() ** _ { rs }

        rule ids() -> Vec<usize>
            = is:integer() ** _ { is }

        pub rule database() -> Database
            = ranges:ranges()
              "\n"
              "\n"
              ids:ids()
                {
                    Database {
                        ranges,
                        ids,
                    }
                }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let database = database::database(input.trim()).map_err(|e| e.to_string())?;

    let fresh = database
        .ids
        .iter()
        .filter(|id| database.ranges.iter().any(|range| range.contains(id)))
        .count();

    Ok(fresh)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut database = database::database(input.trim()).map_err(|e| e.to_string())?;

    database.ranges.sort_unstable_by_key(|range| *range.start());

    let fresh = database
        .ranges
        .into_iter()
        .fold(
            Vec::new(),
            |mut merged: Vec<RangeInclusive<usize>>, range| {
                if let Some(last) = merged.last_mut() {
                    if range.start() <= &last.end().saturating_add(1) {
                        *last = *last.start()..=(*last.end().max(range.end()));
                        return merged;
                    }
                }
                merged.push(range);

                merged
            },
        )
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum();

    Ok(fresh)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(3), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(14), get_part_two(INPUT));
    }
}
