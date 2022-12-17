#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidStateError,
}

#[derive(Copy, Clone)]
enum Rock {
    Flat,
    Cross,
    AnnoyingOne,
    Vertical,
    Cube,
}

impl Rock {
    fn get_shape(input: &str) -> Vec<(isize, isize)> {
        input
            .trim()
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '.' => None,
                    _ => Some((x as isize, y as isize)),
                })
            })
            .collect()
    }

    fn shape(self) -> Vec<(isize, isize)> {
        match self {
            Rock::Flat => Rock::get_shape(r##"####"##),
            Rock::Cross => Rock::get_shape(
                r##".#.
###
.#."##,
            ),
            Rock::AnnoyingOne => Rock::get_shape(
                r##"..#
..#
###"##,
            ),
            Rock::Vertical => Rock::get_shape(
                r##"#
#
#
#"##,
            ),
            Rock::Cube => Rock::get_shape(
                r##"##
##"##,
            ),
        }
    }
}

#[derive(Default)]
struct Chamber {
    columns: [HashMap<isize, bool>; 7],
}

#[derive(Default, Hash, Eq, PartialEq)]
struct ChamberState {
    jet_index: usize,
    rock_index: usize,
    columns_diff_to_max: Vec<isize>,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct RockState {
    rock_index: usize,
    tower_height: isize,
}

impl Chamber {
    fn max_height(&self) -> isize {
        self.columns
            .iter()
            .map(|column| *column.keys().max().unwrap_or(&-1))
            .max()
            .unwrap_or(0)
    }

    fn is_collision(&self, shape: Rock, coord: (isize, isize)) -> bool {
        shape.shape().iter().any(|rock| {
            // "…any movement would cause any part of the rock to move into…"
            let x = coord.0 + rock.0;
            let y = coord.1 + rock.1;

            // "…the walls…"
            !(0..=6).contains(&x)
                    // "…floor…"
                    || y < 0
                    // "…or a stopped rock…"
                    || self.columns[x as usize].contains_key(&y)
        })
    }

    // "If a downward movement would have caused a falling rock to move into the
    // floor…the falling rock stops where it is…"
    fn stop_the_rock(&mut self, shape: Rock, coord: (isize, isize)) {
        for rock in shape.shape() {
            self.columns[(coord.0 + rock.0) as usize].insert(coord.1 + rock.1, true);
        }
    }

    fn get_columns_diff_to_max(&self) -> Vec<isize> {
        let max_height = self.max_height();

        self.columns
            .iter()
            .map(|column| max_height - *column.keys().max().unwrap_or(&-1))
            .collect()
    }

    fn falling_rock(
        &mut self,
        jets: &mut impl Iterator<Item = (usize, isize)>,
        rock: Rock,
    ) -> Result<(), AdventOfCodeError> {
        // "Each rock appears so that its left edge is two units away from the left wall
        // and its bottom edge is three units above the highest rock in the room…"
        let mut rock_position = (2, self.max_height() + 4);

        loop {
            // "…it alternates between being pushed by a jet of hot gas one unit…"
            let motion = jets.next().ok_or(AdventOfCodeError::InvalidStateError)?.1;
            let pushed = (rock_position.0 + motion, rock_position.1);
            if !self.is_collision(rock, pushed) {
                rock_position = (pushed.0, pushed.1);
            }

            // "…and then falling one unit down."
            let one_unit_down = (rock_position.0, rock_position.1 - 1);
            if self.is_collision(rock, one_unit_down) {
                self.stop_the_rock(rock, rock_position);
                break;
            }

            rock_position = one_unit_down;
        }

        Ok(())
    }
}

fn get_tower_height(input: &str, limit: usize) -> Result<isize, AdventOfCodeError> {
    let mut chamber = Chamber::default();

    let mut jets = input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => unreachable!(),
        })
        .enumerate()
        .cycle()
        .peekable();

    let mut rocks = [
        Rock::Flat,
        Rock::Cross,
        Rock::AnnoyingOne,
        Rock::Vertical,
        Rock::Cube,
    ]
    .iter()
    .copied()
    .enumerate()
    .cycle()
    .peekable();

    let mut cache = HashMap::<ChamberState, RockState>::new();

    let mut rock_index = 0;
    while rock_index < limit {
        rock_index += 1;

        chamber.falling_rock(
            &mut jets,
            rocks.next().ok_or(AdventOfCodeError::InvalidStateError)?.1,
        )?;

        // try and find a repeating cycle for Part 2…
        let chamber_state = ChamberState {
            jet_index: jets.peek().ok_or(AdventOfCodeError::InvalidStateError)?.0,
            rock_index: rocks.peek().ok_or(AdventOfCodeError::InvalidStateError)?.0,
            columns_diff_to_max: chamber.get_columns_diff_to_max(),
        };
        let rock_state = RockState {
            rock_index,
            tower_height: chamber.max_height(),
        };

        if let Some(previous_occurrence) = cache.insert(chamber_state, rock_state) {
            let cycle_length = rock_index - previous_occurrence.rock_index;
            let cycles_remaining = (limit - rock_index) / cycle_length;

            rock_index += cycles_remaining * cycle_length;
            let growth_per_cycle = rock_state.tower_height - previous_occurrence.tower_height;
            let total_growth = (cycles_remaining as isize) * growth_per_cycle;

            for _ in rock_index..limit {
                chamber.falling_rock(
                    &mut jets,
                    rocks.next().ok_or(AdventOfCodeError::InvalidStateError)?.1,
                )?;
            }

            return Ok(chamber.max_height() + 1 + total_growth);
        }
    }

    Ok(chamber.max_height() + 1)
}

pub fn get_part_one(input: &str) -> Result<isize, AdventOfCodeError> {
    get_tower_height(input, 2_022)
}

pub fn get_part_two(input: &str) -> Result<isize, AdventOfCodeError> {
    get_tower_height(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(3_068), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(1_514_285_714_288), get_part_two(INPUT));
    }
}
