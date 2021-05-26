use std::collections::HashMap;
use std::str::FromStr;

type Position = (isize, isize);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LumberCollectionArea {
    acres: HashMap<Position, Acre>,
}

impl FromStr for LumberCollectionArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let acres = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, ch)| {
                    (
                        (x as isize, y as isize),
                        match ch {
                            '.' => Acre::OpenGround,
                            '|' => Acre::Trees,
                            '#' => Acre::Lumberyard,
                            _ => panic!(r#"¯\_(ツ)_/¯"#),
                        },
                    )
                })
            })
            .collect();

        Ok(Self { acres })
    }
}

impl ToString for LumberCollectionArea {
    fn to_string(&self) -> String {
        let max = self.acres.keys().max().unwrap();

        (0..=max.1)
            .map(|y| {
                (0..=max.0)
                    .map(|x| match self.acres.get(&(x, y)).unwrap() {
                        Acre::OpenGround => '.',
                        Acre::Trees => '|',
                        Acre::Lumberyard => '#',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl LumberCollectionArea {
    fn get_neighbours(&self, position: &Position) -> Vec<&Acre> {
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (position.0 + dx, position.1 + dy)))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .filter(|neighbour| *neighbour != *position)
            .filter_map(|acre| self.acres.get(&acre))
            .collect()
    }

    fn get_next_minute(&self) -> Self {
        let acres = self
            .acres
            .iter()
            .map(|(position, contents)| {
                let acre = match *contents {
                    Acre::OpenGround => {
                        let trees = self
                            .get_neighbours(&position)
                            .iter()
                            .filter(|acre| matches!(*acre, Acre::Trees))
                            .count();
                        match trees {
                            count if count >= 3 => Acre::Trees,
                            _ => Acre::OpenGround,
                        }
                    }
                    Acre::Trees => {
                        let lumberyards = self
                            .get_neighbours(&position)
                            .iter()
                            .filter(|acre| matches!(*acre, Acre::Lumberyard))
                            .count();
                        match lumberyards {
                            count if count >= 3 => Acre::Lumberyard,
                            _ => Acre::Trees,
                        }
                    }
                    Acre::Lumberyard => {
                        let trees = self
                            .get_neighbours(&position)
                            .iter()
                            .filter(|acre| matches!(*acre, Acre::Trees))
                            .count();
                        let lumberyards = self
                            .get_neighbours(&position)
                            .iter()
                            .filter(|acre| matches!(*acre, Acre::Lumberyard))
                            .count();
                        match (trees, lumberyards) {
                            (trees, lumberyards) if trees >= 1 && lumberyards >= 1 => {
                                Acre::Lumberyard
                            }
                            _ => Acre::OpenGround,
                        }
                    }
                };
                (*position, acre)
            })
            .collect();

        Self { acres }
    }

    fn get_resource_value(&self) -> usize {
        let (trees, lumberyards) = self
            .acres
            .values()
            .map(|acre| match *acre {
                Acre::Lumberyard => (0, 1),
                Acre::Trees => (1, 0),
                _ => (0, 0),
            })
            .fold(
                (0, 0),
                |(total_trees, total_lumberyards), (trees, lumberyards)| {
                    (total_trees + trees, total_lumberyards + lumberyards)
                },
            );

        trees * lumberyards
    }

    pub fn get_resource_value_after_minutes(&self, minutes: usize) -> usize {
        let mut last_seen_at = HashMap::new();
        let mut resource_values = HashMap::new();
        let mut area = self.get_next_minute();

        last_seen_at.insert(area.to_string(), 0);
        resource_values.insert(0, area.get_resource_value());

        for minute in 1..minutes {
            area = area.get_next_minute();

            if last_seen_at.contains_key(&area.to_string()) {
                let cycle_start = *last_seen_at.get(&area.to_string()).unwrap();
                let cycle_length = minute - cycle_start;
                let cycle_at_minutes = (cycle_start - 1) + (minutes - cycle_start) % cycle_length;

                return *resource_values.get(&cycle_at_minutes).unwrap();
            }

            last_seen_at.insert(area.to_string(), minute);
            resource_values.insert(minute, area.get_resource_value());
        }

        area.get_resource_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#;

        let area = LumberCollectionArea::from_str(&input).unwrap();

        assert_eq!(1147, area.get_resource_value_after_minutes(10))
    }
}
