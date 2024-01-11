//! Uses the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
//! and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem).
use num_complex::Complex;

const UP: Complex<isize> = Complex::new(0, -1);
const DOWN: Complex<isize> = Complex::new(0, 1);
const LEFT: Complex<isize> = Complex::new(-1, 0);
const RIGHT: Complex<isize> = Complex::new(1, 0);

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let sketch = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0..sketch.len())
        .find_map(|y| {
            (0..sketch[y].len())
                .find_map(|x| (sketch[y][x] == 'S').then_some(Complex::new(x as isize, y as isize)))
        })
        .ok_or(format!("no start: {}", input))?;

    // TODO: determine?
    let mut direction = RIGHT;
    let mut position = start + direction;
    let mut steps = 1;

    loop {
        while sketch[position.im as usize][position.re as usize] == '-'
            || sketch[position.im as usize][position.re as usize] == '|'
        {
            position += direction;
            steps += 1;
        }

        direction = match sketch[position.im as usize][position.re as usize] {
            '7' if direction == UP => LEFT,
            'F' if direction == UP => RIGHT,
            'J' if direction == DOWN => LEFT,
            'L' if direction == DOWN => RIGHT,
            'J' | 'L' => UP,
            '7' | 'F' => DOWN,
            _ => break,
        };

        position += direction;
        steps += 1;
    }

    Ok(steps / 2)
}

fn determinant(a: Complex<isize>, b: Complex<isize>) -> isize {
    a.re * b.im - a.im * b.re
}

pub fn get_part_two(input: &str) -> Result<isize, String> {
    let sketch = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut corner = (0..sketch.len())
        .find_map(|y| {
            (0..sketch[y].len())
                .find_map(|x| (sketch[y][x] == 'S').then_some(Complex::new(x as isize, y as isize)))
        })
        .ok_or(format!("no start: {}", input))?;

    // TODO: true for tests/input but universally…?
    let mut direction = match sketch[(corner + RIGHT).im as usize][(corner + RIGHT).re as usize] {
        'F' => DOWN,
        _ => RIGHT,
    };
    let mut position = corner + direction;
    let mut area = 0;
    let mut steps = 1;

    loop {
        while sketch[position.im as usize][position.re as usize] == '-'
            || sketch[position.im as usize][position.re as usize] == '|'
        {
            position += direction;
            steps += 1;
        }

        direction = match sketch[position.im as usize][position.re as usize] {
            '7' if direction == UP => LEFT,
            'F' if direction == UP => RIGHT,
            'J' if direction == DOWN => LEFT,
            'L' if direction == DOWN => RIGHT,
            'J' | 'L' => UP,
            '7' | 'F' => DOWN,
            _ => {
                area += determinant(corner, position);
                break;
            }
        };

        area += determinant(corner, position);
        corner = position;
        position += direction;
        steps += 1;
    }

    Ok(area.abs() / 2 - steps / 2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;
    const TWO: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;
    const THREE: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
    const FOUR: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
    const FIVE: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(4), get_part_one(ONE));
        assert_eq!(Ok(8), get_part_one(TWO));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(4), get_part_two(THREE));
        assert_eq!(Ok(8), get_part_two(FOUR));
        assert_eq!(Ok(10), get_part_two(FIVE));
    }
}
