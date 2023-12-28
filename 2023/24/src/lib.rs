type Position = (isize, isize, isize);
type Velocity = (isize, isize, isize);
type Hailstone = (Position, Velocity);

peg::parser! {
    pub grammar trajectories() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> isize
            = n:$("-"?['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule position() -> Position
          = x:integer()
            "," _
            y:integer()
            "," _
            z:integer()
            {
              (x, y, z)
            }

        rule hailstone() -> Hailstone
          = position_:position()
            _ "@" _
            velocity:position()
            {
                (position_, velocity)
            }

        pub rule hailstones() -> Vec<Hailstone>
          = hailstones:hailstone() ++ _
            {
                hailstones
            }
    }
}

fn intersects(
    &((xa, ya, _), (dxa, dya, _)): &Hailstone,
    &((xb, yb, _), (dxb, dyb, _)): &Hailstone,
) -> Option<(f64, f64)> {
    let (xa, ya, dxa, dya, xb, yb, dxb, dyb) = (
        xa as f64, ya as f64, dxa as f64, dya as f64, xb as f64, yb as f64, dxb as f64, dyb as f64,
    );
    let (x2, y2, x4, y4) = (xa + dxa, ya + dya, xb + dxb, yb + dyb);

    let ua = ((xa - xb) * (yb - y4) - (ya - yb) * (xb - x4))
        / ((xa - x2) * (yb - y4) - (ya - y2) * (xb - x4));
    let ub = ((xa - xb) * (ya - y2) - (ya - yb) * (xa - x2))
        / ((xa - x2) * (yb - y4) - (ya - y2) * (xb - x4));

    if ua > 0.0 && ub > 0.0 {
        Some((xa + dxa * ua, ya + dya * ua))
    } else {
        None
    }
}

pub fn get_part_one(input: &str, min: f64, max: f64) -> Result<usize, String> {
    let hailstones = trajectories::hailstones(input.trim()).map_err(|e| e.to_string())?;

    let intersections = hailstones
        .iter()
        .enumerate()
        .map(|(i, a)| {
            hailstones
                .iter()
                .skip(i)
                .filter(|b| match intersects(a, b) {
                    Some((x, y)) => (min..=max).contains(&x) && (min..=max).contains(&y),
                    _ => false,
                })
                .count()
        })
        .sum();

    Ok(intersections)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(2), get_part_one(INPUT, 7.0, 27.0));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
