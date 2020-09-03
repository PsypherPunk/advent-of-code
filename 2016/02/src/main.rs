use std::cmp;
use std::fs;

type Point = (isize, isize);

const BUTTONS: [[&str; 3]; 3] = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];
const NEW_BUTTONS: [[Option<&str>; 5]; 5] = [
    [None, None, Some("1"), None, None],
    [None, Some("2"), Some("3"), Some("4"), None],
    [Some("5"), Some("6"), Some("7"), Some("8"), Some("9")],
    [None, Some("A"), Some("B"), Some("C"), None],
    [None, None, Some("D"), None, None],
];

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

fn get_new_code(input: &str) -> String {
    let mut position: Point = (0, 2);

    input
        .lines()
        .map(|line| {
            line.chars().for_each(|direction| {
                position = match direction {
                    'U' => {
                        if NEW_BUTTONS[cmp::max(0, position.1 - 1) as usize][position.0 as usize]
                            .is_some()
                        {
                            (position.0, cmp::max(0, position.1 - 1))
                        } else {
                            position
                        }
                    }
                    'D' => {
                        if NEW_BUTTONS[cmp::min(4, position.1 + 1) as usize][position.0 as usize]
                            .is_some()
                        {
                            (position.0, cmp::min(4, position.1 + 1))
                        } else {
                            position
                        }
                    }
                    'L' => {
                        if NEW_BUTTONS[position.1 as usize][cmp::max(0, position.0 - 1) as usize]
                            .is_some()
                        {
                            (cmp::max(0, position.0 - 1), position.1)
                        } else {
                            position
                        }
                    }
                    'R' => {
                        if NEW_BUTTONS[position.1 as usize][cmp::min(4, position.0 + 1) as usize]
                            .is_some()
                        {
                            (cmp::min(4, position.0 + 1), position.1)
                        } else {
                            position
                        }
                    }
                    _ => panic!("Invalid direction: {}", direction),
                };
            });

            NEW_BUTTONS[position.1 as usize][position.0 as usize].unwrap()
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("What is the bathroom code? {}", get_code(&input),);

    println!("…what is the correct bathroom code? {}", get_new_code(&input));
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

    #[test]
    fn test_two() {
        let input = r#"ULL
RRDDD
LURDL
UUUUD
"#;
        assert_eq!("5DB3", get_new_code(&input));
    }
}
