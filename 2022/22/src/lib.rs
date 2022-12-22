const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(PartialEq, Clone, Debug, Copy)]
enum Tile {
    Open,
    Wall,
    Void,
}

#[derive(Debug, Clone)]
enum Step {
    Turn(usize),
    Steps(isize),
}

pub struct Notes {
    map: Vec<Vec<Tile>>,
    steps: Vec<Step>,
}

peg::parser! {
    pub grammar monkeys() for str {
        rule _() = [' ']*

        rule __() = ['\n']*

        rule number() -> isize
            = n:$(['0'..='9']+) {? n.parse().or(Err("number()")) }

        rule direction() -> usize
            = t:$(['L' | 'R'])
                {
                    match t {
                        "L" => 3,
                        "R" => 1,
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
            = steps:number() { Step::Steps(steps) } / direction:direction() { Step::Turn(direction) }

        rule steps() -> Vec<Step>
            = steps:step() ++ ""

        pub rule notes() -> Notes
            = map:map() __ __ steps:steps()
                {
                    let width = map.iter().map(|row| row.len()).max().unwrap();

                    let map = map.into_iter()
                        .map(|row| {
                            let len = row.len();
                            [row, vec![Tile::Void; width - len]].concat()
                        })
                        .collect::<Vec<_>>();

                    Notes {
                        map,
                        steps,
                    }
                }
    }
}

impl Notes {
    // TODO: overflowing all over the place!
    fn wrap(&self, mut row: usize, mut column: usize, direction: usize) -> (usize, usize) {
        let (dr, dc) = DIRECTIONS[direction];
        while *self
            .map
            .get(row - dr as usize)
            .and_then(|row| row.get(column - dc as usize))
            .unwrap_or(&Tile::Void)
            != Tile::Void
        {
            (row, column) = (row - dr as usize, column - dc as usize);
        }
        (row, column)
    }
}

pub fn get_part_one(input: &str) -> usize {
    let notes = monkeys::notes(input.trim_end()).unwrap();

    let (mut row, mut column, mut direction) = (0, 0, 1);

    while notes.map[0][column] != Tile::Open {
        column += 1
    }

    for step in &notes.steps {
        match step {
            Step::Turn(turn) => direction = (direction + turn) % 4,
            Step::Steps(count) => {
                for _ in 0..*count {
                    let (dr, dc) = DIRECTIONS[direction];
                    let next = notes
                        .map
                        .get(row + dr as usize)
                        .and_then(|row| row.get(column + dc as usize))
                        .unwrap_or(&Tile::Void);

                    match next {
                        Tile::Open => (row, column) = (row + dr as usize, column + dc as usize),
                        Tile::Wall => break,
                        Tile::Void => {
                            let (next_row, next_column) = notes.wrap(row, column, direction);
                            if notes.map[next_row][next_column] == Tile::Wall {
                                break;
                            }
                            (row, column) = (next_row, next_column);
                        }
                    }
                }
            }
        }
    }

    1000 * (row + 1) + 4 * (column + 1) + [3, 0, 1, 2][direction]
}

fn wrap_cube(row: usize, column: usize, direction: usize) -> (usize, usize) {
    // dbg!(&row, &column, &direction);
    let (cube_row, cube_column, new_direction) = match (row / 50, column / 50, direction) {
        (0, 1, 0) => (3, 0, 1),
        (0, 1, 3) => (2, 0, 1),
        (0, 2, 0) => (3, 0, 0),
        (0, 2, 1) => (2, 1, 3),
        (0, 2, 2) => (1, 1, 3),
        (1, 1, 1) => (0, 2, 0),
        (1, 1, 3) => (2, 0, 2),
        (2, 0, 0) => (1, 1, 1),
        (2, 0, 3) => (0, 1, 1),
        (2, 1, 1) => (0, 2, 3),
        (2, 1, 2) => (3, 0, 3),
        (3, 0, 1) => (2, 1, 0),
        (3, 0, 2) => (0, 2, 2),
        (3, 0, 3) => (0, 1, 2),
        _ => unreachable!(),
    };
    let (row_on_face, column_on_face) = (row % 50, column % 50);
    let i = [
        column_on_face,
        row_on_face,
        49 - column_on_face,
        49 - row_on_face,
    ][direction];
    let (new_row, new_column) = [(49, i), (i, 0), (0, 49 - i), (49 - i, 49)][new_direction];

    (cube_row * 50 + new_row, cube_column * 50 + new_column)
}

pub fn get_part_two(input: &str) -> usize {
    let notes = monkeys::notes(input.trim_end()).unwrap();

    let (mut row, mut column, mut direction) = (0, 0, 1);

    while notes.map[0][column] != Tile::Open {
        column += 1
    }

    for step in &notes.steps {
        match step {
            Step::Turn(turn) => direction = (direction + turn) % 4,
            Step::Steps(count) => {
                for _ in 0..*count {
                    let (dr, dc) = DIRECTIONS[direction];
                    let next = notes
                        .map
                        .get(row + dr as usize)
                        .and_then(|row| row.get(column + dc as usize))
                        .unwrap_or(&Tile::Void);

                    match next {
                        Tile::Open => (row, column) = (row + dr as usize, column + dc as usize),
                        Tile::Wall => break,
                        Tile::Void => {
                            let (next_row, next_column) = wrap_cube(row, column, direction);
                            if notes.map[next_row][next_column] == Tile::Wall {
                                break;
                            }
                            (row, column) = (next_row, next_column);
                        }
                    }
                }
            }
        }
    }

    1000 * (row + 1) + 4 * (column + 1) + [3, 0, 1, 2][direction]
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

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(5_031, get_part_two(INPUT));
    // }
}
