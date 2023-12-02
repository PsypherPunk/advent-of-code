#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

peg::parser! {
    pub grammar game() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule cube() -> (usize, &'input str)
            = count:integer()
              _
              colour:$("red" / "green" / "blue")
              { (count, colour) }

        rule set() -> Set
            = cubes:cube() ++ ", "
              {
                Set {
                    red: cubes.iter().find_map(|(count, colour)| if *colour == "red" { Some(*count) } else { None }).unwrap_or(0),
                    green: cubes.iter().find_map(|(count, colour)| if *colour == "green" { Some(*count) } else { None }).unwrap_or(0),
                    blue: cubes.iter().find_map(|(count, colour)| if *colour == "blue" { Some(*count) } else { None }).unwrap_or(0),
                }
              }

        rule game() -> Game
            = "Game "
              id:integer()
              ": "
              sets:set() ++ "; "
              _
              {
                Game {
                    id,
                    sets,
                }
              }

        pub rule games() -> Vec<Game>
            = games:game() ++ _
            {
                games
            }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    Ok(game::games(input)
        .map_err(|e| e.to_string())?
        .iter()
        .filter_map(|game| {
            match game
                .sets
                .iter()
                .find(|set| set.red > 12 || set.green > 13 || set.blue > 14)
            {
                Some(_) => None,
                None => Some(game.id),
            }
        })
        .sum())
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(8), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
