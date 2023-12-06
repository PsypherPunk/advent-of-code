struct Race {
    time: usize,
    distance: usize,
}

peg::parser! {
    pub grammar paper() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule digits() -> &'input str
            = n:$(['0'..='9']+)

        pub rule races() -> Vec<Race>
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
                        Race {
                            time: *time,
                            distance,
                        }
                    })
                    .collect()
                }

        pub rule race_rtfm() -> Race
            = "Time:"
              _
              times:digits() ++ _
              _
              "Distance:"
              _
              distances:digits() ++ _
              {?
                let time = times.join("").parse().or(Err("invalid integer"))?;
                let distance = distances.join("").parse().or(Err("invalid integer"))?;

                Ok(Race {
                    time,
                    distance,
                })
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

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let race = paper::race_rtfm(input.trim()).map_err(|e| format!("bad input: {}", e))?;

    let time = race.time as f64;
    let distance = race.distance as f64;
    let sqrt = (time * time - 4.0 * (distance + 1.0)).sqrt();
    let ways = (((time + sqrt) / 2.0).floor() - ((time - sqrt) / 2.0).ceil()) as usize + 1;

    Ok(ways)
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
        assert_eq!(Ok(71503), get_part_two(INPUT));
    }
}
