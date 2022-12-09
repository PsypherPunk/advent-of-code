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

#[allow(unused)]
pub fn get_part_two(input: &str) -> usize {
    0
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

    #[test]
    fn test_part_one() {
        assert_eq!(13, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
