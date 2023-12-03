#[derive(Debug)]
enum Position {
    Number(usize, usize),
    Symbol(usize, usize),
}

peg::parser! {
    pub grammar schematic() for str {
        rule _() = ['.' | '\n']*

        rule integer() -> Position
            = start:position!() n:$(['0'..='9']+) end:position!()
                {?
                    match n.parse::<usize>() {
                        Ok(n) => Ok(Position::Number(start, end)),
                        Err(e) => Err("invalid integer"),
                    }
                }

        rule symbol() -> Position
            = start:position!() $(['=' | '&' | '@' | '+' | '*' | '%' | '/' | '$' | '#' | '-']) end:position!()
                { Position::Symbol(start, end) }

        pub rule symbols() -> Vec<Position>
            = _
              positions:(symbol() / integer()) ++ _
              _
                { positions }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let line_length = input
        .char_indices()
        .find_map(|(position, c)| match c == '\n' {
            true => Some(position + 1), // inc. newline char.
            false => None,
        })
        .ok_or_else(|| "no lines?!".to_owned())?;
    let positions = schematic::symbols(input.trim()).map_err(|e| e.to_string())?;

    let part_numbers = positions
        .iter()
        .filter_map(|position| match position {
            Position::Symbol(n, _) => Some((n % line_length, n / line_length)),
            _ => None,
        })
        .flat_map(|(sx, sy)| {
            positions
                .iter()
                .filter_map(|position| match position {
                    Position::Number(start, end) => Some((start, end)),
                    _ => None,
                }) // get number start/end.
                .filter(move |&(start, end)| {
                    sy.abs_diff(start / line_length) <= 1 // y is within 1…
                        && ((start % line_length).saturating_sub(1)..((end % line_length) + 1))
                            .contains(&sx) // x-range is within 1.
                })
                .map(|(start, end)| input[*start..*end].parse::<usize>())
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(part_numbers.iter().sum())
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(4361), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
