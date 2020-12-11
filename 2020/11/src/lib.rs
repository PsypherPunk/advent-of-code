use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Clone, PartialEq)]
pub struct SeatLayout {
    positions: HashMap<Point, Position>,
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
                                _ => panic!("Unexpected character: {}", char),
                            },
                        )
                    })
                    .collect::<Vec<(Point, Position)>>()
            })
            .collect();

        Ok(Self { positions })
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

    fn get_occupied_neighbour_count(&self, point: &Point) -> usize {
        let (px, py) = point;
        let (max_x, max_y) = self.positions.keys().max().unwrap();

        (py.saturating_sub(1)..=py + 1)
            .flat_map(|ny| {
                (px.saturating_sub(1)..=px + 1)
                    .map(move |nx| (nx, ny))
                    .collect::<Vec<Point>>()
            })
            .filter(|neighbour| *neighbour != *point)
            .filter(|(nx, ny)| nx <= max_x && ny <= max_y)
            .filter(|neighbour| {
                matches!(
                    self.positions.get(neighbour).unwrap(),
                    Position::OccupiedSeat
                )
            })
            .count()
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
                    n if n >= 4 => (*point, Position::EmptySeat),
                    _ => (*point, Position::OccupiedSeat),
                },
                _ => (*point, position),
            })
            .collect();

        SeatLayout { positions }
    }
}

pub fn get_stable_layout(seat_layout: &SeatLayout) -> SeatLayout {
    let mut current = seat_layout.clone();
    loop {
        let next = current.get_next_layout();

        println!("{}", &current);

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
}
