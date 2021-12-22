use std::collections::HashSet;

#[derive(Debug)]
pub struct RebootStep {
    on: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

peg::parser! {
    pub grammar reactor() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> isize
            = n:$("-"* ['0'..='9']+)
                {? n.parse().or(Err("Invalid integer.")) }

        rule on_off() -> bool
            = on:$(['a'..='z']+)
                { on == "on" }

        pub rule step() -> RebootStep
            = on:on_off()
              _
              "x=" x_start:integer() ".." x_end:integer()
              ","
              "y=" y_start:integer() ".." y_end:integer()
              ","
              "z=" z_start:integer() ".." z_end:integer()
                    {
                        RebootStep {
                            on,
                            x: (x_start, x_end),
                            y: (y_start, y_end),
                            z: (z_start, z_end),
                        }
                    }

        pub rule steps() -> Vec<RebootStep>
            = steps:step() ++ _
                { steps }
    }
}

pub fn get_part_one(input: &str) -> usize {
    let steps = reactor::steps(input.trim()).unwrap();

    let mut cubes = HashSet::new();

    for step in steps {
        if step.x.0 < -50 || step.x.1 > 50 {
            continue;
        }
        for x in step.x.0..=step.x.1 {
            for y in step.y.0..=step.y.1 {
                for z in step.z.0..=step.z.1 {
                    match step.on {
                        true => cubes.insert((x, y, z)),
                        false => cubes.remove(&(x, y, z)),
                    };
                }
            }
        }
    }

    cubes.len()
}

pub fn get_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use parameterized::parameterized;

    #[parameterized(input = {
        "on x=10..12,y=10..12,z=10..12",
        r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13"#,
        r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11"#,
        r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#,
        r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"#,
    }, count = {
        27, 46, 38, 39, 590_784,
    })]
    fn test_part_one(input: &str, count: usize) {
        assert_eq!(count, get_part_one(input));
    }

    #[test]
    fn test_part_two() {
        // assert_eq!(2, get_part_two(INPUT));
    }
}
