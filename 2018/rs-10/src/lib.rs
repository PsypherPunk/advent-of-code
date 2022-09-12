use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::str::FromStr;

use num::complex::Complex;

pub struct PointOfLight {
    position: Complex<isize>,
    velocity: Complex<isize>,
}

pub struct Sky {
    points_of_light: Vec<PointOfLight>,
}

peg::parser! {
    pub grammar sky() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> isize
            = n:$("-"* ['0'..='9']+)
                {? n.parse().or(Err("Invalid integer.")) }

        rule position() -> Complex<isize>
            = "position=<"
              _
              x:integer()
              ", "
              _
              y:integer()
              ">"
                  {
                      Complex::new(x, y)
                  }

        rule velocity() -> Complex<isize>
            = "velocity=<"
              _
              dx:integer()
              ", "
              _
              dy:integer()
              ">"
                  {
                      Complex::new(dx, dy)
                  }

        rule point_of_light() -> PointOfLight
            = position:position()
              _
              velocity:velocity()
                {
                    PointOfLight {
                        position,
                        velocity,
                    }
                }

        pub rule sky() -> Sky
            = points_of_light:point_of_light() ++ _
                { Sky { points_of_light }}
    }
}

impl FromStr for Sky {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        sky::sky(s.trim()).map_err(|err| err.to_string())
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        let positions = self
            .points_of_light
            .iter()
            .map(|point_of_light| (point_of_light.position.re, point_of_light.position.im))
            .collect::<BTreeSet<(isize, isize)>>();

        let min_x = positions.iter().map(|(x, _)| *x).min().unwrap_or(0);
        let min_y = positions.iter().map(|(_, y)| *y).min().unwrap_or(0);
        let max_x = positions.iter().map(|(x, _)| *x).max().unwrap_or(150);
        let max_y = positions.iter().map(|(_, y)| *y).max().unwrap_or(150);

        let output = (min_y..=max_y)
            .map(|y| {
                let positions = &positions;
                (min_x..=max_x)
                    .map(move |x| match positions.contains(&(x, y)) {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

impl Sky {
    fn lit_area(&self) -> isize {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for point_of_light in &self.points_of_light {
            min_x = min_x.min(point_of_light.position.re);
            min_y = min_y.min(point_of_light.position.im);
            max_x = max_x.max(point_of_light.position.re);
            max_y = max_y.max(point_of_light.position.im);
        }

        (max_x - min_x) * (max_y - min_y)
    }

    fn tick(&self) -> Self {
        let points_of_light = self
            .points_of_light
            .iter()
            .map(|point_of_light| PointOfLight {
                position: point_of_light.position + point_of_light.velocity,
                velocity: point_of_light.velocity,
            })
            .collect();

        Self { points_of_light }
    }
}

pub fn get_part_one(input: &str) -> Result<String, String> {
    let mut sky = Sky::from_str(input.trim())?;

    let mut last_lit_area = None;

    loop {
        let next = sky.tick();
        if last_lit_area.is_some() && next.lit_area() > sky.lit_area() {
            break;
        }

        sky = next;
        last_lit_area = Some(sky.lit_area());
    }

    Ok(sky.to_string())
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut sky = Sky::from_str(input.trim())?;
    let mut seconds = 0;
    let mut last_lit_area = None;

    loop {
        let next = sky.tick();
        if last_lit_area.is_some() && next.lit_area() > sky.lit_area() {
            break;
        }

        seconds += 1;
        sky = next;
        last_lit_area = Some(sky.lit_area());
    }

    Ok(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
"#;

    #[test]
    fn test_part_one() {
        let result = get_part_one(INPUT);

        let expected = r##"#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###"##;

        assert_eq!(Ok(expected.to_string()), result);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(3), get_part_two(INPUT));
    }
}
