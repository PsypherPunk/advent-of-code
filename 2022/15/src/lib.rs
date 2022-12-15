use peg::error::ParseError;
use peg::str::LineCol;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidSensorsError(ParseError<LineCol>),
    PleaseMindTheGapError,
}

impl From<ParseError<LineCol>> for AdventOfCodeError {
    fn from(error: ParseError<LineCol>) -> Self {
        AdventOfCodeError::InvalidSensorsError(error)
    }
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhanttan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
pub struct Sensor {
    position: Point,
    beacon: Point,
}

#[derive(Clone, Debug)]
struct Range {
    start: isize,
    end: isize,
}

peg::parser! {
    pub grammar sensors() for str {

        rule _() = [' ' | '\n']*

        rule number() -> isize
            = n:$("-"?['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule point() -> Point
            = "x="
            x:number()
            ", y="
            y:number()
                {
                    Point {
                        x,
                        y
                    }
                }

        rule sensor() -> Sensor
            = "Sensor at "
              position:point()
              ": closest beacon is at "
              beacon:point()
                {
                    Sensor {
                        position,
                        beacon,
                    }
                }



        pub rule sensors() -> Vec<Sensor>
            = _ s:sensor() ++ _
                { s }
    }
}

pub fn get_part_one(input: &str, y: isize) -> Result<isize, AdventOfCodeError> {
    let sensors = sensors::sensors(input.trim())?;

    let mut ranges = sensors
        .iter()
        .filter(|sensor| {
            let distance_to_beacon = sensor.position.manhanttan_distance(&sensor.beacon);

            sensor.position.y.abs_diff(y) <= distance_to_beacon
        })
        .map(|sensor| {
            let distance_to_beacon = sensor.position.manhanttan_distance(&sensor.beacon);
            let distance_to_y = sensor.position.y.abs_diff(y);

            let diff = distance_to_beacon.abs_diff(distance_to_y) as isize;

            Range {
                start: sensor.position.x - diff,
                end: sensor.position.x + diff,
            }
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by_key(|range| range.start);

    let mut overlapped = Vec::new();
    let mut current = ranges[0].clone();
    for next in ranges[1..].iter() {
        if next.start > current.end {
            overlapped.push(current.clone());
        } else if current.end < next.end {
            current.end = next.end;
        }
    }
    overlapped.push(current);

    Ok(overlapped.iter().map(|range| range.end - range.start).sum())
}

pub fn get_part_two(input: &str) -> Result<isize, AdventOfCodeError> {
    let sensors = sensors::sensors(input.trim())?;

    let (y, gap) = (0..=4_000_000)
        .into_par_iter()
        .find_map_any(|y| {
            let mut ranges = sensors
                .iter()
                .filter(|sensor| {
                    let distance_to_beacon = sensor.position.manhanttan_distance(&sensor.beacon);

                    sensor.position.y.abs_diff(y) <= distance_to_beacon
                })
                .map(|sensor| {
                    let distance_to_beacon = sensor.position.manhanttan_distance(&sensor.beacon);
                    let distance_to_y = sensor.position.y.abs_diff(y);

                    let diff = distance_to_beacon.abs_diff(distance_to_y) as isize;

                    Range {
                        start: sensor.position.x - diff,
                        end: sensor.position.x + diff,
                    }
                })
                .collect::<Vec<_>>();

            ranges.sort_unstable_by_key(|range| range.start);

            let mut current_end = ranges[0].end;
            for next in ranges[1..].iter() {
                if next.start > current_end {
                    return Some((y, current_end + 1));
                }
                if current_end < next.end {
                    current_end = next.end;
                }
            }

            None
        })
        .ok_or(AdventOfCodeError::PleaseMindTheGapError)?;

    Ok((gap * 4000000) + y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(26), get_part_one(INPUT, 10));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(56_000_011), get_part_two(INPUT));
    }
}
