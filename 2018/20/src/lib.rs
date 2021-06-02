use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Default)]
pub struct Map {
    rooms: HashMap<Position, HashSet<Position>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Map::default();

        let mut current = Position::default();
        map.rooms.entry(current).or_insert_with(HashSet::new);

        let mut branches = Vec::new();

        for c in s.trim().chars() {
            match c {
                '(' => {
                    branches.push(current);
                }
                '|' => {
                    current = *branches.last().unwrap();
                }
                ')' => {
                    current = branches.pop().unwrap();
                }
                'N' => {
                    let next = Position {
                        x: current.x,
                        y: current.y - 1,
                    };
                    map.add_doorway(current, next);
                    current = next;
                }
                'S' => {
                    let next = Position {
                        x: current.x,
                        y: current.y + 1,
                    };
                    map.add_doorway(current, next);
                    current = next;
                }
                'W' => {
                    let next = Position {
                        x: current.x - 1,
                        y: current.y,
                    };
                    map.add_doorway(current, next);
                    current = next;
                }
                'E' => {
                    let next = Position {
                        x: current.x + 1,
                        y: current.y,
                    };
                    map.add_doorway(current, next);
                    current = next;
                }
                '^' | '$' => {}
                _ => panic!("Invalid character: {}", c),
            }
        }

        Ok(map)
    }
}

impl Map {
    fn add_doorway(&mut self, a: Position, b: Position) {
        let neighbours = self.rooms.entry(a).or_insert_with(HashSet::new);
        neighbours.insert(b);

        let neighbours = self.rooms.entry(b).or_insert_with(HashSet::new);
        neighbours.insert(a);
    }

    pub fn get_most_doors(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let mut max_doors = 0;
        queue.push_back((Position::default(), 0));

        while !queue.is_empty() {
            let (current, doors) = queue.pop_front().unwrap();

            visited.insert(current);
            max_doors = max_doors.max(doors);

            self.rooms
                .get(&current)
                .unwrap()
                .iter()
                .filter(|room| !visited.contains(room))
                .for_each(|neighbour| {
                    queue.push_back((*neighbour, doors + 1));
                });
        }

        max_doors
    }

    pub fn get_rooms_over_1000_doors_away(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let mut rooms = 0;
        queue.push_back((Position::default(), 0));

        while !queue.is_empty() {
            let (current, doors) = queue.pop_front().unwrap();

            visited.insert(current);
            if doors >= 1000 {
                rooms += 1;
            }

            self.rooms
                .get(&current)
                .unwrap()
                .iter()
                .filter(|room| !visited.contains(room))
                .for_each(|neighbour| {
                    queue.push_back((*neighbour, doors + 1));
                });
        }

        rooms
    }
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        map = {
            "^WNE$",
            "^ENWWW(NEEE|SSE(EE|N))$",
            "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
            "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$",
            "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
        },
        doors = {
            3,
            10,
            18,
            23,
            31,
        }
    )]
    fn test_part_one(map: &str, doors: usize) {
        assert_eq!(Map::from_str(&map).unwrap().get_most_doors(), doors);
    }
}
