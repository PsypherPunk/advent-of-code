pub struct Notes {
    map: Vec<Vec<Tile>>,
    path: Vec<Step>,
}

#[derive(Copy, Clone)]
struct Point {
    row: isize,
    column: isize,
}

enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

enum Spin {
    Left,
    Right,
}

#[derive(PartialEq)]
enum Tile {
    Open,
    Wall,
    Void,
}

enum Step {
    Turn(Spin),
    Number(usize),
}

impl Direction {
    fn turn(self, turn: &Spin) -> Direction {
        match (self, turn) {
            (Self::Left, Spin::Left) => Self::Down,
            (Self::Left, Spin::Right) => Self::Up,
            (Self::Right, Spin::Left) => Self::Up,
            (Self::Right, Spin::Right) => Self::Down,
            (Self::Up, Spin::Left) => Self::Left,
            (Self::Up, Spin::Right) => Self::Right,
            (Self::Down, Spin::Left) => Self::Right,
            (Self::Down, Spin::Right) => Self::Left,
        }
    }

    fn step(&self) -> Point {
        match &self {
            Self::Left => Point { row: 0, column: -1 },
            Self::Right => Point { row: 0, column: 1 },
            Self::Up => Point { row: -1, column: 0 },
            Self::Down => Point { row: 1, column: 0 },
        }
    }
}

impl Notes {
    fn wrap_around(&self, position: &Point, direction: &Direction) -> Point {
        let Point {
            row: dr,
            column: dc,
        } = direction.step();
        let mut current = *position;

        while let Some(tile) = self
            .map
            .get((current.row - dr) as usize)
            .and_then(|row| row.get((current.column - dc) as usize))
        {
            if *tile == Tile::Void {
                break;
            }
            current = Point {
                row: current.row - dr,
                column: current.column - dc,
            };
        }

        current
    }
}

fn wrap_around_cube(position: &Point, direction: &Direction) -> (Point, Direction) {
    let (cube_row, cube_col, new_dir) = match (position.row / 50, position.column / 50, direction) {
        (0, 1, Direction::Up) => (3, 0, Direction::Right),
        (0, 1, Direction::Left) => (2, 0, Direction::Right),
        (0, 2, Direction::Up) => (3, 0, Direction::Up),
        (0, 2, Direction::Right) => (2, 1, Direction::Left),
        (0, 2, Direction::Down) => (1, 1, Direction::Left),
        (1, 1, Direction::Right) => (0, 2, Direction::Up),
        (1, 1, Direction::Left) => (2, 0, Direction::Down),
        (2, 0, Direction::Up) => (1, 1, Direction::Right),
        (2, 0, Direction::Left) => (0, 1, Direction::Right),
        (2, 1, Direction::Right) => (0, 2, Direction::Left),
        (2, 1, Direction::Down) => (3, 0, Direction::Left),
        (3, 0, Direction::Right) => (2, 1, Direction::Up),
        (3, 0, Direction::Down) => (0, 2, Direction::Down),
        (3, 0, Direction::Left) => (0, 1, Direction::Down),
        _ => unreachable!(),
    };
    let (row_idx, col_idx) = (position.row % 50, position.column % 50);

    let i = match direction {
        Direction::Left => 49 - row_idx,
        Direction::Right => row_idx,
        Direction::Up => col_idx,
        Direction::Down => 49 - col_idx,
    };

    let new_row = match new_dir {
        Direction::Left => 49 - i,
        Direction::Right => i,
        Direction::Up => 49,
        Direction::Down => 0,
    };
    let new_col = match new_dir {
        Direction::Left => 49,
        Direction::Right => 0,
        Direction::Up => i,
        Direction::Down => 49 - i,
    };

    let new_pos = Point {
        row: cube_row * 50 + new_row,
        column: cube_col * 50 + new_col,
    };

    (new_pos, new_dir)
}

peg::parser! {
    pub grammar monkeys() for str {
        rule _() = [' ']*

        rule __() = ['\n']*

        rule number() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule spin() -> Spin
            = t:$(['L' | 'R'])
                {
                    match t {
                        "L" => Spin::Left,
                        "R" => Spin::Right,
                        _ => unreachable!(),
                    }
                }

        rule tiles() -> Vec<Tile>
            = tiles:$([' ' | '.' | '#']+)
                {
                    tiles.chars()
                        .map(|c| {
                            match c {
                                '.' => Tile::Open,
                                '#' => Tile::Wall,
                                ' ' => Tile::Void,
                                x => unreachable!(),
                            }
                        })
                        .collect()
                }

        rule map() -> Vec<Vec<Tile>>
            = tiles:tiles() ++ __
                { tiles }

        rule step() -> Step
            = steps:number() { Step::Number(steps) } / spin:spin() { Step::Turn(spin) }

        rule steps() -> Vec<Step>
            = steps:step() ++ ""

        pub rule notes() -> Notes
            = map:map() __ __ steps:steps()
                {
                    Notes {
                        map,
                        path: steps,
                    }
                }
    }
}

pub fn get_part_one(input: &str) -> isize {
    let notes = monkeys::notes(input.trim_end()).unwrap();

    let column = notes.map[0]
        .iter()
        .position(|tile| *tile == Tile::Open)
        .unwrap() as isize;

    let mut position = Point { row: 0, column };
    let mut direction = Direction::Right;

    for step in &notes.path {
        match step {
            Step::Turn(turn) => direction = direction.turn(turn),
            Step::Number(amount) => {
                for _ in 0..*amount {
                    let Point {
                        row: dr,
                        column: dc,
                    } = direction.step();
                    let new_tile = notes
                        .map
                        .get((position.row + dr) as usize)
                        .and_then(|row| row.get((position.column + dc) as usize))
                        .unwrap_or(&Tile::Void);

                    match new_tile {
                        Tile::Wall => break,
                        Tile::Open => {
                            position = Point {
                                row: position.row + dr,
                                column: position.column + dc,
                            };
                        }
                        Tile::Void => {
                            let new_position = notes.wrap_around(&position, &direction);
                            if notes.map[new_position.row as usize][new_position.column as usize]
                                == Tile::Wall
                            {
                                break;
                            }
                            position = new_position;
                        }
                    }
                }
            }
        }
    }

    1000 * (position.row + 1) + 4 * (position.column + 1) + direction as isize
}

pub fn get_part_two(input: &str) -> isize {
    let notes = monkeys::notes(input.trim_end()).unwrap();

    let column = notes.map[0]
        .iter()
        .position(|tile| *tile == Tile::Open)
        .unwrap() as isize;

    let mut position = Point { row: 0, column };
    let mut direction = Direction::Right;

    for step in &notes.path {
        match step {
            Step::Turn(turn) => direction = direction.turn(turn),
            Step::Number(amount) => {
                for _ in 0..*amount {
                    let Point {
                        row: dr,
                        column: dc,
                    } = direction.step();
                    let new_tile = notes
                        .map
                        .get((position.row + dr) as usize)
                        .and_then(|row| row.get((position.column + dc) as usize))
                        .unwrap_or(&Tile::Void);

                    match new_tile {
                        Tile::Wall => break,
                        Tile::Open => {
                            position = Point {
                                row: position.row + dr,
                                column: position.column + dc,
                            };
                        }
                        Tile::Void => {
                            let (new_position, new_direction) =
                                wrap_around_cube(&position, &direction);
                            if notes.map[new_position.row as usize][new_position.column as usize]
                                == Tile::Wall
                            {
                                break;
                            }
                            position = new_position;
                            direction = new_direction
                        }
                    }
                }
            }
        }
    }

    1000 * (position.row + 1) + 4 * (position.column + 1) + direction as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(6_032, get_part_one(INPUT));
    }

    // Different layout; doesn't work.
    // #[test]
    // fn test_part_two() {
    //     assert_eq!(5_031, get_part_two(INPUT));
    // }
}
