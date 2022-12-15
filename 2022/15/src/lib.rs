use std::collections::HashSet;

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

pub fn get_part_one(input: &str, y: isize) -> usize {
    let sensors = sensors::sensors(input.trim()).unwrap();

    let beacons_x = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == y)
        .map(|sensor| sensor.beacon.x)
        .collect::<HashSet<_>>()
        .len();
    
    sensors
        .iter()
        .filter(|sensor| {
            let distance_to_beacon = sensor.position.manhanttan_distance(&sensor.beacon);

            sensor.position.y.abs_diff(y) <= distance_to_beacon
        })
        .flat_map(|sensor| {
            let distance_to_beacon = sensor.position.manhanttan_distance(&sensor.beacon);
            let distance_to_y = sensor.position.y.abs_diff(y);

            let diff = distance_to_beacon.abs_diff(distance_to_y) as isize;

            sensor.position.x - diff..=sensor.position.x + diff
        })
        .collect::<HashSet<_>>()
        .len() - beacons_x
}

pub fn get_part_two(input: &str) -> usize {

    0
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
        assert_eq!(26, get_part_one(INPUT, 10));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
