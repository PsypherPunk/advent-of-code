use std::collections::BTreeSet;

peg::parser! {
    pub grammar scratchcard() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule card() -> usize
            = "Card"
              _
              integer()
              ":"
              _
              winning:integer() ++ _
              _ "|" _
              player:integer() ++ _
              {
                let winning = winning
                    .iter()
                    .collect::<BTreeSet<_>>();
                let player = player
                    .iter()
                    .collect::<BTreeSet<_>>();

                winning.intersection(&player).count()
              }

        pub rule cards() -> Vec<usize>
            = cards:card() ++ _
              {
                cards
              }

    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let scratchcards = scratchcard::cards(input.trim()).map_err(|e| e.to_string())?;

    let points = scratchcards
        .iter()
        .filter_map(|winning_count| match *winning_count > 0 {
            true => Some(1 << (winning_count - 1)),
            false => None,
        })
        .sum();

    Ok(points)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(13), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
