pub struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

peg::parser! {
    pub grammar target() for str {
        rule integer() -> isize
            = n:$("-"* ['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        pub rule target() -> Target
            = "target area: x=" x_min:integer() ".." x_max:integer() ", y=" y_min:integer() ".." y_max:integer()
                {
                    Target {
                        x_min,
                        x_max,
                        y_min,
                        y_max,
                    }
                }
    }
}

pub fn get_part_one(input: &str) -> isize {
    let target = target::target(input.trim()).unwrap();

    let (mut x, mut y, mut max_y) = (0, 0, 0);

    let ys = (target.y_min..=1000)
        .flat_map(|mut dy| {
            (0..=target.x_max)
                .map(move |mut dx| loop {
                    x += dx;
                    y += dy;
                    dx -= dx.signum();
                    dy -= 1;
                    max_y = max_y.max(y);

                    match (
                        target.x_min <= x && x <= target.x_max,
                        target.y_min <= y && y <= target.y_max,
                    ) {
                        (true, true) => return Some(max_y),
                        (false, _) if dx == 0 => return None,
                        (_, false) if y < target.y_min => return None,
                        _ => {}
                    }
                })
                .flatten()
        })
        .collect::<Vec<_>>();

    *ys.iter().max().unwrap()
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"target area: x=20..30, y=-10..-5"#;

    #[test]
    fn test_part_one() {
        assert_eq!(45, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
