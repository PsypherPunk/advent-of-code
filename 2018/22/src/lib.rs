use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Rocky,
    Narrow,
    Wet,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tool {
    Torch,
    ClimbingGear,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Move {
    region: Coordinate,
    tool: Option<Tool>,
    minutes: usize,
    distance_to_target: usize,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.minutes + self.distance_to_target).cmp(&(other.minutes + other.distance_to_target))
    }
}

type Coordinate = (usize, usize);

pub struct Cave {
    depth: usize,
    erosion: HashMap<Coordinate, usize>,
    target: Coordinate,
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines().collect::<Vec<_>>();
        let depth = lines[0].split_whitespace().last().unwrap().parse().unwrap();
        let target = lines[1]
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|numbers| numbers.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            depth,
            erosion: HashMap::new(),
            target: (target[0], target[1]),
        })
    }
}

impl Cave {
    fn get_manhattan_distance(&self, a: &Coordinate, b: &Coordinate) -> usize {
        ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
    }

    fn get_neighbouring_regions(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        let (x, y) = coordinate;

        vec![
            (*x, y.saturating_sub(1)),
            (x + 1, *y),
            (*x, y + 1),
            (x.saturating_sub(1), *y),
        ]
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
    }

    fn get_type(&mut self, coordinate: &Coordinate) -> Type {
        match self.get_erosion_level(&coordinate) % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => unreachable!(),
        }
    }

    fn get_erosion_level(&mut self, coordinate: &Coordinate) -> usize {
        if self.erosion.contains_key(&coordinate) {
            return *self.erosion.get(&coordinate).unwrap();
        }

        let geologic_index = self.get_geologic_index(&coordinate);
        let erosion = (geologic_index + self.depth) % 20_183;

        self.erosion.insert(*coordinate, erosion);

        erosion
    }

    fn get_geologic_index(&mut self, coordinate: &Coordinate) -> usize {
        match coordinate {
            (0, 0) => 0,
            target if *target == self.target => 0,
            (x, 0) => *x * 16_807,
            (0, y) => *y * 48_271,
            (x, y) => self.get_erosion_level(&(x - 1, *y)) * self.get_erosion_level(&(*x, y - 1)),
        }
    }

    pub fn get_risk_level(&mut self) -> usize {
        (0..=self.target.1)
            .map(|y| {
                (0..=self.target.0)
                    .map(|x| match self.get_type(&(x, y)) {
                        Type::Rocky => 0,
                        Type::Wet => 1,
                        Type::Narrow => 2,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn get_is_tool_ok_for_region(&mut self, tool: &Option<Tool>, region: &Coordinate) -> bool {
        match self.get_type(region) {
            Type::Rocky => {
                matches!(tool, Some(Tool::ClimbingGear)) || matches!(tool, Some(Tool::Torch))
            }
            Type::Wet => matches!(tool, Some(Tool::ClimbingGear)) || matches!(tool, None),
            Type::Narrow => matches!(tool, Some(Tool::Torch)) || matches!(tool, None),
        }
    }

    fn get_tools_ok_for_region(&mut self, region: &Coordinate) -> Vec<Option<Tool>> {
        match self.get_type(region) {
            Type::Rocky => vec![Some(Tool::ClimbingGear), Some(Tool::Torch)],
            Type::Wet => vec![Some(Tool::ClimbingGear), None],
            Type::Narrow => vec![Some(Tool::Torch), None],
        }
    }

    fn get_right_tool(
        &mut self,
        current: &Coordinate,
        next: &Coordinate,
    ) -> Result<Option<Tool>, String> {
        let next_tools = self.get_tools_ok_for_region(next);

        self.get_tools_ok_for_region(current)
            .into_iter()
            .find(|tool| next_tools.contains(tool))
            .ok_or_else(|| "Unable to find the right tool".to_owned())
    }

    pub fn get_quickest_time_to_target(&mut self) -> Result<usize, String> {
        let mut queue = BinaryHeap::new();
        let mut discovered = HashSet::new();

        queue.push(Reverse(Move {
            region: (0, 0),
            tool: Some(Tool::Torch),
            minutes: 0,
            distance_to_target: self.get_manhattan_distance(&(0, 0), &self.target),
        }));

        while !queue.is_empty() {
            let current = queue.pop().unwrap().0;
            discovered.insert((current.region, current.tool));

            if current.region == self.target {
                return match current.tool {
                    Some(Tool::Torch) => Ok(current.minutes),
                    _ => Ok(current.minutes + 7),
                };
            }

            let next_moves = self
                .get_neighbouring_regions(&current.region)
                .iter()
                .map(|neighbouring_region| {
                    let current_region_type = self.get_type(&current.region);
                    let neighbouring_region_type = self.get_type(neighbouring_region);
                    if neighbouring_region_type == current_region_type
                        || self.get_is_tool_ok_for_region(&current.tool, &neighbouring_region)
                    {
                        Move {
                            region: *neighbouring_region,
                            tool: current.tool,
                            minutes: current.minutes + 1,
                            distance_to_target: self
                                .get_manhattan_distance(&neighbouring_region, &self.target),
                        }
                    } else {
                        Move {
                            region: *neighbouring_region,
                            tool: self
                                .get_right_tool(&current.region, neighbouring_region)
                                .unwrap(),
                            minutes: current.minutes + 8,
                            distance_to_target: self
                                .get_manhattan_distance(&neighbouring_region, &self.target),
                        }
                    }
                })
                .filter(|next| !discovered.contains(&(next.region, next.tool)))
                .collect::<Vec<_>>();
            for next_move in next_moves {
                queue.push(Reverse(next_move));
            }
        }

        Err("No path to target found.".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut cave = Cave {
            depth: 510,
            erosion: HashMap::new(),
            target: (10, 10),
        };

        assert_eq!(0, cave.get_geologic_index(&(0, 0)));
        assert_eq!(510, cave.get_erosion_level(&(0, 0)));
        assert_eq!(Type::Rocky, cave.get_type(&(0, 0)));
        assert_eq!(16_807, cave.get_geologic_index(&(1, 0)));
        assert_eq!(17_317, cave.get_erosion_level(&(1, 0)));
        assert_eq!(Type::Wet, cave.get_type(&(1, 0)));
        assert_eq!(48_271, cave.get_geologic_index(&(0, 1)));
        assert_eq!(8_415, cave.get_erosion_level(&(0, 1)));
        assert_eq!(Type::Rocky, cave.get_type(&(0, 1)));
        assert_eq!(145_722_555, cave.get_geologic_index(&(1, 1)));
        assert_eq!(1_805, cave.get_erosion_level(&(1, 1)));
        assert_eq!(Type::Narrow, cave.get_type(&(1, 1)));
        assert_eq!(0, cave.get_geologic_index(&(10, 10)));
        assert_eq!(510, cave.get_erosion_level(&(10, 10)));
        assert_eq!(Type::Rocky, cave.get_type(&(10, 10)));
        assert_eq!(114, cave.get_risk_level());
    }

    #[test]
    fn test_part_two() {
        let mut cave = Cave {
            depth: 510,
            erosion: HashMap::new(),
            target: (10, 10),
        };

        assert_eq!(45, cave.get_quickest_time_to_target().unwrap());
    }
}
