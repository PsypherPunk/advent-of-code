use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Species {
    Elf,
    Goblin,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Creature {
    position: Point,
    hit_points: usize,
    attack_power: usize,
    species: Species,
}

impl Creature {
    pub fn is_alive(&self) -> bool {
        self.hit_points > 0
    }
}

impl Default for Creature {
    fn default() -> Self {
        Creature {
            position: (0, 0),
            hit_points: 200,
            attack_power: 3,
            species: Species::Elf,
        }
    }
}

#[derive(Clone)]
pub struct Cave {
    walls: HashSet<Point>,
    creatures: Vec<Creature>,
}

impl FromStr for Cave {
    type Err = String;

    /// Derive a `Cave` from our input.
    ///
    /// The list of `Creature` is stored as a `Vec` so it can be sorted
    /// as per the reading-order (note: positions are stored as
    /// `(y, x)`, not `(x, y)` so sorting prioritised `y`.)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut creatures = Vec::new();
        let mut walls = HashSet::new();

        s.trim().lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| match ch {
                '#' => {
                    walls.insert((y, x));
                }
                'E' => {
                    creatures.push(Creature {
                        position: (y, x),
                        species: Species::Elf,
                        ..Default::default()
                    });
                }
                'G' => {
                    creatures.push(Creature {
                        position: (y, x),
                        species: Species::Goblin,
                        ..Default::default()
                    });
                }
                '.' => {}
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            });
        });

        Ok(Self { walls, creatures })
    }
}

impl Cave {
    fn manhattan(a: Point, b: Point) -> usize {
        ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
    }

    fn get_move(&self, unit: usize) -> Option<Point> {
        let enemies = self
            .creatures
            .iter()
            .filter(|creature| {
                creature.species != self.creatures[unit].species && creature.is_alive()
            })
            .map(|creature| creature.position)
            .collect::<HashSet<_>>();

        let occupied = self
            .creatures
            .iter()
            .filter(|creature| creature.is_alive())
            .map(|creature| creature.position)
            .collect::<HashSet<_>>();

        let mut queue = VecDeque::new();
        let mut route: HashMap<Point, Point> = HashMap::new();

        queue.push_back((0, self.creatures[unit].position));

        while let Some((d, (y, x))) = queue.pop_front() {
            let steps = vec![(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)]
                .into_iter()
                .filter(|step| !route.contains_key(step) && !self.walls.contains(step))
                .collect::<Vec<_>>();

            for step in &steps {
                if enemies.contains(step) {
                    return route.remove(&(y, x));
                }
                if !occupied.contains(step) {
                    let previous_step = *route.get(&(y, x)).unwrap_or(step);
                    route.insert(*step, previous_step);
                    queue.push_back((d + 1, *step));
                }
            }
        }
        None
    }

    fn get_attack(&self, unit: usize) -> Option<usize> {
        self.creatures
            .iter()
            .enumerate()
            .filter(|(_, creature)| {
                creature.species != self.creatures[unit].species && creature.is_alive()
            })
            .filter(|(_, creature)| {
                Cave::manhattan(creature.position, self.creatures[unit].position) == 1
            })
            .min_by(|&(_, a), &(_, b)| {
                a.hit_points
                    .cmp(&b.hit_points)
                    .then(a.position.cmp(&b.position))
            })
            .map(|(position, _)| position)
    }

    fn get_alive_goblin_count(&self) -> usize {
        self.creatures
            .iter()
            .filter(|creature| matches!(creature.species, Species::Goblin) && creature.is_alive())
            .count()
    }

    fn get_elf_count(&self) -> usize {
        self.creatures
            .iter()
            .filter(|creature| matches!(creature.species, Species::Elf))
            .count()
    }

    fn get_alive_elf_count(&self) -> usize {
        self.creatures
            .iter()
            .filter(|creature| matches!(creature.species, Species::Elf) && creature.is_alive())
            .count()
    }

    fn get_round(&mut self) -> bool {
        self.creatures.sort();

        for i in 0..self.creatures.len() {
            if !self.creatures[i].is_alive() {
                continue;
            }

            if self.get_alive_elf_count() == 0 || self.get_alive_goblin_count() == 0 {
                return false;
            }

            if let Some(new_pos) = self.get_move(i) {
                self.creatures[i].position = new_pos;
            }

            if let Some(target) = self.get_attack(i) {
                let attack_power = self.creatures[i].attack_power;
                let target = &mut self.creatures[target];
                target.hit_points = target.hit_points.saturating_sub(attack_power);
            }
        }

        true
    }

    fn get_score(&self, rounds: usize) -> usize {
        rounds
            * self
                .creatures
                .iter()
                .map(|creature| creature.hit_points as usize)
                .sum::<usize>()
    }

    pub fn get_outcome(&mut self) -> usize {
        let mut rounds = 0;

        while self.get_round() {
            rounds += 1;
        }

        self.get_score(rounds)
    }
}

pub fn get_outcome_no_losses(input: &str) -> usize {
    let original = Cave::from_str(&input).unwrap();

    let mut rounds;
    let score;
    let mut elf_attack_power = 4;
    let elf_count = original.get_elf_count();

    loop {
        let mut cave = original.clone();
        rounds = 0;

        for i in 0..cave.creatures.len() {
            if matches!(cave.creatures[i].species, Species::Elf) {
                cave.creatures[i].attack_power = elf_attack_power;
            }
        }

        while cave.get_round() {
            rounds += 1;
        }

        if cave.get_alive_elf_count() == elf_count {
            score = cave.get_score(rounds);
            break;
        }
        elf_attack_power += 1;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#;

        let mut cave = Cave::from_str(&input).unwrap();

        assert_eq!(27_730, cave.get_outcome());
    }

    #[test]
    fn test_part_two() {
        let input = r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#;

        assert_eq!(4_988, get_outcome_no_losses(&input));
    }
}
