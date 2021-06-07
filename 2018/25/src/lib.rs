use std::collections::BTreeSet;

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    t: isize,
    constellation: usize,
}

fn get_manhattan_distance(a: &Point, b: &Point) -> isize {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) + (a.t - b.t).abs()
}

fn get_points(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (x, y, z, t) = match line.trim().split(',').collect::<Vec<_>>()[..] {
                [a, b, c, d] => (
                    a.parse().unwrap(),
                    b.parse().unwrap(),
                    c.parse().unwrap(),
                    d.parse().unwrap(),
                ),
                _ => unreachable!(),
            };
            Point {
                x,
                y,
                z,
                t,
                constellation: i,
            }
        })
        .collect()
}

pub fn get_constellation_count(input: &str) -> usize {
    let mut points = get_points(&input);

    for a in 0..points.len() {
        for b in a + 1..points.len() {
            let a = points[a];
            let b = points[b];

            if get_manhattan_distance(&a, &b) <= 3 && a.constellation != b.constellation {
                points.iter_mut().for_each(|c| {
                    if c.constellation == b.constellation {
                        c.constellation = a.constellation;
                    }
                });
            }
        }
    }

    points
        .iter()
        .map(|point| point.constellation)
        .collect::<BTreeSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let input = r#" 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0"#;

        assert_eq!(2, get_constellation_count(&input));
    }

    #[test]
    fn test_4() {
        let input = r#"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"#;

        assert_eq!(4, get_constellation_count(&input));
    }

    #[test]
    fn test_3() {
        let input = r#"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"#;

        assert_eq!(3, get_constellation_count(&input));
    }

    #[test]
    fn test_8() {
        let input = r#"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"#;

        assert_eq!(8, get_constellation_count(&input));
    }
}
