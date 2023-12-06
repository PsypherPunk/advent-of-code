struct Races {
    time: usize,
    distance: usize,
}

peg::parser! {
    pub grammar paper() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        pub rule races() -> Vec<Races>
            = "Time:"
              _
              times:integer() ++ _
              _
              "Distance:"
              _
              distances:integer() ++ _
              {
                times
                    .iter()
                    .zip(distances)
                    .map(|(time, distance)| {
                        Races {
                            time: *time,
                            distance,
                        }
                    })
                    .collect()
              }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let races = paper::races(input.trim()).map_err(|e| format!("bad input: {}", e))?;

    let product_of_ways = races
        .iter()
        .map(|race| {
            (0..=race.time)
                .filter(|hold| ((race.time - hold) * hold) > race.distance)
                .count()
        })
        .product();

    Ok(product_of_ways)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(288), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
