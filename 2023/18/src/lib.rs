use itertools::Itertools;

pub fn get_part_one(input: &str) -> Result<isize, String> {
    let dig_plan = input
        .trim()
        .lines()
        .map(|line| {
            let (direction, suffix) = line.split_once(' ').ok_or(format!("bad line: {}", line))?;
            let (metres, _) = suffix
                .split_once(' ')
                .ok_or(format!("bad line: {}", line))?;
            let metres = metres.parse::<isize>().map_err(|e| e.to_string())?;

            Ok::<(_, _), String>((direction, metres))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let positions = dig_plan
        .iter()
        .scan((0, 0), |position, (direction, metres)| {
            *position = match *direction {
                "U" => (position.0, position.1 - metres),
                "D" => (position.0, position.1 + metres),
                "L" => (position.0 - metres, position.1),
                "R" => (position.0 + metres, position.1),
                _ => unreachable!(),
            };

            Some((position.0, position.1))
        })
        .collect::<Vec<_>>();

    // TODO: `concat()` otherwise the perimeter misses the first step…?
    let (area, perimeter) = [vec![(0, 0)], positions]
        .concat()
        .iter()
        .tuple_windows()
        .fold((0, 0), |(area, perimeter), ((ax, ay), (bx, by))| {
            let area = area + (ax * by) - (ay * bx);
            let perimeter = if ax == bx {
                perimeter + ay.abs_diff(*by) as isize
            } else {
                perimeter + ax.abs_diff(*bx) as isize
            };

            (area, perimeter)
        });

    Ok(((area / 2) - (perimeter / 2) + 1) + perimeter)
}

pub fn get_part_two(input: &str) -> Result<isize, String> {
    let dig_plan = input
        .trim()
        .lines()
        .map(|line| {
            let (_, hex) = line.split_once('#').ok_or(format!("bad line: {}", line))?;
            let metres = isize::from_str_radix(&hex[..5], 16).map_err(|e| e.to_string())?;

            Ok::<(_, _), String>((&hex[5..=5], metres))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let positions = dig_plan
        .iter()
        .scan((0, 0), |position, (direction, metres)| {
            *position = match *direction {
                "3" => (position.0, position.1 - metres),
                "1" => (position.0, position.1 + metres),
                "2" => (position.0 - metres, position.1),
                "0" => (position.0 + metres, position.1),
                _ => unreachable!(),
            };

            Some((position.0, position.1))
        })
        .collect::<Vec<_>>();

    // TODO: otherwise the perimeter misses the first step…?
    let (area, perimeter) = [vec![(0, 0)], positions]
        .concat()
        .iter()
        .tuple_windows()
        .fold((0, 0), |(area, perimeter), ((ax, ay), (bx, by))| {
            let area = area + (ax * by) - (ay * bx);
            let perimeter = if ax == bx {
                perimeter + ay.abs_diff(*by) as isize
            } else {
                perimeter + ax.abs_diff(*bx) as isize
            };

            (area, perimeter)
        });

    Ok(((area / 2) - (perimeter / 2) + 1) + perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(62), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(952408144115), get_part_two(INPUT));
    }
}
