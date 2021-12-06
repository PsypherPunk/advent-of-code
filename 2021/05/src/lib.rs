use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Vent {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct LineSegment {
    start: Vent,
    end: Vent,
}

impl LineSegment {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

peg::parser! {
    pub grammar vents() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> isize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule line_segment() -> LineSegment
            = x1:integer() "," y1:integer() _ "->" _ x2:integer() "," y2:integer()
                {
                    match (x1, y1).cmp(&(x2, y2)) {
                        Ordering::Greater => LineSegment {
                            start: Vent {
                                x: x2,
                                y: y2,
                            },
                            end: Vent { x: x1, y: y1 }

                            },
                        _ => LineSegment {
                            start: Vent {
                                x: x1,
                                y: y1,
                            },
                            end: Vent { x: x2, y: y2 }
                        }
                        ,
                    }
                }

        pub rule vents() -> Vec<LineSegment>
            = line_segments:line_segment() ++ _
              _
                { line_segments }
    }
}

pub fn get_part_one(input: &str) -> usize {
    let line_segments = vents::vents(input).unwrap();

    line_segments
        .iter()
        .flat_map(|segment| {
            if segment.is_horizontal() {
                (segment.start.x..=segment.end.x)
                    .map(|x| (x, segment.start.y))
                    .collect()
            } else if segment.is_vertical() {
                (segment.start.y..=segment.end.y)
                    .map(|y| (segment.start.x, y))
                    .collect()
            } else {
                vec![]
            }
        })
        .fold(
            HashMap::<(isize, isize), usize>::new(),
            |mut counts, position| {
                *counts.entry(position).or_default() += 1;
                counts
            },
        )
        .values()
        .filter(|&count| *count > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(5, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, 2)
    }
}
