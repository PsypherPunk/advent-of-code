use std::cmp;
use std::fs;

type Point = (isize, isize);

const BUTTONS: [[&str; 3]; 3] = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];

#[allow(dead_code)]
fn get_button_coord(button: isize) -> Point {
    ((button - 1) % 3, ((button - 1) / 3))
}

fn get_code(input: &str) -> String {
    let mut position: Point = (1, 1);

    input
        .lines()
        .map(|line| {
            line.chars().for_each(|direction| {
                match direction {
                    'U' => position.1 = cmp::max(0, position.1 - 1),
                    'D' => position.1 = cmp::min(2, position.1 + 1),
                    'L' => position.0 = cmp::max(0, position.0 - 1),
                    'R' => position.0 = cmp::min(2, position.0 + 1),
                    _ => panic!("Invalid direction: {}", direction),
                };
            });

            BUTTONS[position.1 as usize][position.0 as usize]
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the bathroom code? {}", get_code(&input),);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord() {
        assert_eq!((0, 0), get_button_coord(1));
        assert_eq!((1, 1), get_button_coord(5));
        assert_eq!((2, 2), get_button_coord(9));
    }

    #[test]
    fn test_u() {
        assert_eq!("2", get_code("U"));
    }

    #[test]
    fn test_one() {
        let input = r#"ULL
RRDDD
LURDL
UUUUD
"#;
        assert_eq!("1985", get_code(&input));
    }
}
