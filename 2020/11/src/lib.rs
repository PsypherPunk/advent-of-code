use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

type Point = (usize, usize);
type Direction = (isize, isize);

const LINE_OF_SIGHT: [Direction; 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeatLayout {
    positions: HashMap<Point, Position>,
    tolerance: usize,
}

impl FromStr for SeatLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        (
                            (x, y),
                            match char {
                                '.' => Position::Floor,
                                'L' => Position::EmptySeat,
                                '#' => Position::OccupiedSeat,
                                _ => panic!("Unexpected character: {}", char),
                            },
                        )
                    })
                    .collect::<Vec<(Point, Position)>>()
            })
            .collect();

        Ok(Self {
            positions,
            tolerance: 4,
        })
    }
}

impl Display for SeatLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let (max_x, max_y) = self.positions.keys().max().unwrap();

        let output = (0..=*max_y)
            .map(|y| {
                (0..=*max_x)
                    .map(|x| match self.positions.get(&(x, y)).unwrap() {
                        Position::OccupiedSeat => '#',
                        Position::EmptySeat => 'L',
                        Position::Floor => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

impl SeatLayout {
    pub fn get_occupied_seat_count(&self) -> usize {
        self.positions
            .values()
            .filter(|position| matches!(position, Position::OccupiedSeat))
            .count()
    }

    pub fn set_tolerance(&mut self, tolerance: usize) {
        self.tolerance = tolerance;
    }

    fn get_occupied_neighbour_count(&self, point: &Point) -> usize {
        let (px, py) = point;

        (py.saturating_sub(1)..=py + 1)
            .map(|ny| {
                (px.saturating_sub(1)..=px + 1)
                    .map(move |nx| (nx, ny))
                    .filter(|neighbour| *neighbour != *point)
                    .filter(|neighbour| {
                        matches!(self.positions.get(neighbour), Some(Position::OccupiedSeat))
                    })
                    .count()
            })
            .sum()
    }

    fn get_visible_neighbour_count(&self, point: &Point) -> usize {
        let (px, py) = point;

        LINE_OF_SIGHT
            .iter()
            .map(|(los_x, los_y)| {
                let mut visible_seats = 0;
                let (mut view_x, mut view_y) = ((*px as isize + los_x), (*py as isize + los_y));
                while view_x >= 0 && view_y >= 0 {
                    match self.positions.get(&(view_x as usize, view_y as usize)) {
                        Some(Position::OccupiedSeat) => {
                            visible_seats = 1;
                            break;
                        }
                        None | Some(Position::EmptySeat) => {
                            break;
                        }
                        _ => {
                            view_x += los_x;
                            view_y += los_y;
                        }
                    }
                }
                visible_seats
            })
            .sum()
    }

    fn get_next_layout(&self) -> SeatLayout {
        let positions = self
            .positions
            .iter()
            .map(|(point, &position)| match position {
                Position::EmptySeat => match self.get_occupied_neighbour_count(point) {
                    0 => (*point, Position::OccupiedSeat),
                    _ => (*point, Position::EmptySeat),
                },
                Position::OccupiedSeat => match self.get_occupied_neighbour_count(point) {
                    n if n >= self.tolerance => (*point, Position::EmptySeat),
                    _ => (*point, Position::OccupiedSeat),
                },
                _ => (*point, position),
            })
            .collect();

        SeatLayout {
            positions,
            tolerance: self.tolerance,
        }
    }

    fn get_next_visible_layout(&self) -> SeatLayout {
        let positions = self
            .positions
            .iter()
            .map(|(point, &position)| match position {
                Position::EmptySeat => match self.get_visible_neighbour_count(point) {
                    0 => (*point, Position::OccupiedSeat),
                    _ => (*point, Position::EmptySeat),
                },
                Position::OccupiedSeat => match self.get_visible_neighbour_count(point) {
                    n if n >= self.tolerance => (*point, Position::EmptySeat),
                    _ => (*point, Position::OccupiedSeat),
                },
                _ => (*point, position),
            })
            .collect();

        SeatLayout {
            positions,
            tolerance: self.tolerance,
        }
    }
}

pub fn get_stable_layout(seat_layout: &SeatLayout) -> SeatLayout {
    let mut current = seat_layout.clone();
    loop {
        let next = current.get_next_layout();

        println!("{}\n", &current);

        if next == current {
            return next;
        }
        current = next.clone();
    }
}

pub fn get_correct_stable_layout(seat_layout: &SeatLayout) -> SeatLayout {
    let mut current = seat_layout.clone();
    loop {
        let next = current.get_next_visible_layout();

        println!("{}\n", &current);

        if next == current {
            return next;
        }
        current = next.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    #[test]
    fn test_part_one() {
        let seat_layout = SeatLayout::from_str(&INPUT).unwrap();

        assert_eq!(
            37,
            get_stable_layout(&seat_layout).get_occupied_seat_count(),
        );
    }

    #[test]
    fn test_part_two_8() {
        let input = r#".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."#;
        let seat_layout = SeatLayout::from_str(&input).unwrap();

        assert_eq!(
            Position::EmptySeat,
            *seat_layout.positions.get(&(3, 4)).unwrap(),
        );
        assert_eq!(8, seat_layout.get_visible_neighbour_count(&(3, 4)),);
    }

    #[test]
    fn test_part_two_0() {
        let input = r#".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##."#;
        let seat_layout = SeatLayout::from_str(&input).unwrap();

        assert_eq!(
            Position::EmptySeat,
            *seat_layout.positions.get(&(3, 3)).unwrap(),
        );
        assert_eq!(0, seat_layout.get_visible_neighbour_count(&(3, 3)),);
    }

    #[test]
    fn test_part_two_problem_step() {
        let before = r#"#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"#;
        let after = r#"#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#"#;
        let before = SeatLayout::from_str(&before).unwrap();
        let after = SeatLayout::from_str(&after).unwrap();

        assert_eq!(after, before.get_next_visible_layout());
    }

    #[test]
    fn test_part_two() {
        let mut seat_layout = SeatLayout::from_str(&INPUT).unwrap();
        seat_layout.set_tolerance(5);

        assert_eq!(
            26,
            get_correct_stable_layout(&seat_layout).get_occupied_seat_count(),
        );
    }
}
