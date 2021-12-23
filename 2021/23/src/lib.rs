use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq)]
struct Situation {
    cost: usize,
    rooms: [Vec<char>; 4],
    hallway: [char; 11],
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Situation {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.rooms.hash(hasher);
        self.hallway.hash(hasher);
    }
}

impl PartialOrd<Self> for Situation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Situation {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn is_organised(rooms: &[Vec<char>; 4]) -> bool {
    !rooms.iter().enumerate().any(|(i, room)| {
        room.iter()
            .any(|&c| c == '.' || get_room_for_amphipod(c) != i)
    })
}

fn is_room_organised(i: usize, room: &[char]) -> bool {
    room.iter()
        .all(|&c| c == '.' || get_room_for_amphipod(c) == i)
}

fn get_room_for_amphipod(c: char) -> usize {
    ((c as u8) - b'A') as usize
}

fn get_step_count_to(start: usize, end: usize, hallway: [char; 11]) -> Option<usize> {
    let range = match end.cmp(&start) {
        Ordering::Greater => (start + 1)..(end + 1),
        _ => end..start,
    };

    match hallway[range].iter().find(|&c| *c != '.') {
        Some(_) => None,
        None => Some((end as i32 - start as i32).abs() as usize),
    }
}

fn get_steps_for_room(amphipod: char, room: &[char]) -> Option<usize> {
    match room
        .iter()
        .find(|&other| *other != '.' && *other != amphipod)
    {
        Some(_) => None,
        None => room
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &c)| c == '.')
            .map(|(steps, _)| steps + 1),
    }
}

fn get_cost_for_amphipod(c: char) -> usize {
    10_usize.pow(get_room_for_amphipod(c) as u32)
}

fn get_room_position(room: usize) -> usize {
    2 + (2 * room)
}

fn get_amphipod_to_room(situation: &Situation) -> Option<Situation> {
    for (step, &c) in situation.hallway.iter().enumerate() {
        if c == '.' {
            continue;
        }
        let target_room = get_room_for_amphipod(c);
        if let Some(steps_in) = get_steps_for_room(c, &situation.rooms[target_room]) {
            if let Some(steps_to) =
                get_step_count_to(step, get_room_position(target_room), situation.hallway)
            {
                let mut hallway = situation.hallway;
                let mut rooms = situation.rooms.clone();
                let cost = situation.cost + get_cost_for_amphipod(c) * (steps_to + steps_in);

                hallway[step] = '.';
                rooms[target_room][steps_in - 1] = c;

                return Some(Situation {
                    cost,
                    rooms,
                    hallway,
                });
            }
        }
    }
    None
}

fn get_amphipod_to_hallway(situation: &Situation, room_index: usize) -> Vec<Situation> {
    let mut next = Vec::new();

    let (i, &amphipod) = situation.rooms[room_index]
        .iter()
        .enumerate()
        .find(|&(_, &c)| c != '.')
        .unwrap();
    for h in [0, 1, 3, 5, 7, 9, 10] {
        if let Some(s_move) = get_step_count_to(get_room_position(room_index), h, situation.hallway)
        {
            let mut hallway = situation.hallway;
            let mut rooms = situation.rooms.clone();
            let cost = situation.cost + get_cost_for_amphipod(amphipod) * (s_move + i + 1);

            hallway[h] = amphipod;
            rooms[room_index][i] = '.';

            next.push(Situation {
                cost,
                rooms,
                hallway,
            })
        }
    }

    next
}

fn get_least_energy(rooms: [Vec<char>; 4]) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let situation = Situation {
        cost: 0,
        rooms,
        hallway: ['.'; 11],
    };
    heap.push(situation);

    while let Some(situation) = heap.pop() {
        if seen.contains(&situation) {
            continue;
        }

        seen.insert(situation.clone());

        if is_organised(&situation.rooms) {
            return situation.cost;
        }

        if let Some(next) = get_amphipod_to_room(&situation) {
            heap.push(next);
        }

        for (room_index, room) in situation.rooms.iter().enumerate() {
            if is_room_organised(room_index, room) {
                continue;
            }
            heap.extend(get_amphipod_to_hallway(&situation, room_index));
        }
    }
    unreachable!()
}

pub fn get_part_one(input: &str) -> usize {
    let mut rooms: [Vec<char>; 4] = Default::default();

    input.lines().for_each(|line| {
        line.chars()
            .filter(|c| c.is_alphabetic())
            .enumerate()
            .for_each(|(room, c)| {
                rooms[room].push(c);
            });
    });

    get_least_energy(rooms)
}

pub fn get_part_two(input: &str) -> usize {
    let mut lines = input.lines().collect::<Vec<_>>();
    let insert = r##"  #D#C#B#A#
  #D#B#A#C#
"##
    .lines()
    .collect::<Vec<_>>();

    lines.splice(3..3, insert);

    let mut rooms: [Vec<char>; 4] = Default::default();

    lines.iter().for_each(|line| {
        line.chars()
            .filter(|c| c.is_alphabetic())
            .enumerate()
            .for_each(|(room, c)| {
                rooms[room].push(c);
            });
    });

    get_least_energy(rooms)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########  "#;

    #[test]
    fn test_part_one() {
        assert_eq!(12521, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(44169, get_part_two(INPUT));
    }
}
