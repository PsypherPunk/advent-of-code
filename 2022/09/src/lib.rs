use std::collections::HashSet;

use num_complex::Complex;

#[allow(unused)]
pub fn get_part_one(input: &str) -> usize {
    let up = Complex::new(0, -1);
    let down = Complex::new(0, 1);
    let right = Complex::new(1, 0);
    let left = Complex::new(-1, 0);

    // [(current, previous)]
    let mut rope = vec![
        (Complex::new(0, 0), Complex::new(0, 0)),
        (Complex::new(0, 0), Complex::new(0, 0)),
    ];

    let mut seen = HashSet::new();
    seen.insert(Complex::new(0, 0));

    input.trim().lines().for_each(|line| {
        let (direction, distance) = line.split_once(' ').unwrap();
        let direction = match direction {
            "U" => up,
            "L" => left,
            "D" => down,
            "R" => right,
            _ => unreachable!(),
        };
        let distance = distance.parse::<i32>().unwrap();

        for _ in 0..distance {
            rope[0] = (rope[0].0 + direction, rope[0].0);

            let tug: Complex<i32> = rope[1].0 - rope[0].0;
            if tug.re.abs() > 1 || tug.im.abs() > 1 {
                rope[1] = (rope[0].1, rope[1].0);
                seen.insert(rope[1].0);
            }
        }
    });

    seen.len()
}

fn get_sign(i: i32) -> i32 {
    match i {
        0 => 0,
        i if i > 0 => 1,
        _ => -1,
    }
}

#[allow(unused)]
pub fn get_part_two(input: &str) -> usize {
    let up = Complex::new(0, -1);
    let down = Complex::new(0, 1);
    let right = Complex::new(1, 0);
    let left = Complex::new(-1, 0);

    // [(current, previous)]
    let mut rope = vec![
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
        Complex::new(0, 0),
    ];

    let mut seen = HashSet::new();
    seen.insert(Complex::new(0, 0));

    input.trim().lines().for_each(|line| {
        let (direction, distance) = line.split_once(' ').unwrap();
        let direction = match direction {
            "U" => up,
            "L" => left,
            "D" => down,
            "R" => right,
            _ => unreachable!(),
        };
        let distance = distance.parse::<i32>().unwrap();

        for _ in 0..distance {
            rope[0] += direction;

            for i in 1..10 {
                let tug: Complex<i32> = rope[i - 1] - rope[i];

                if tug.re.abs() > 1 || tug.im.abs() > 1 {
                    rope[i] += Complex::new(get_sign(tug.re), get_sign(tug.im));
                }
            }
            seen.insert(rope[9]);
        }
    });

    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    const LARGER_INPUT: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(13, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, get_part_two(INPUT));
    }

    #[test]
    fn test_part_two_larger() {
        assert_eq!(36, get_part_two(LARGER_INPUT));
    }
}
