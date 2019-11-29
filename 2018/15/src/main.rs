use std::cmp::{Reverse, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
//use std::{thread, time};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Species {
    Goblin,
    Elf,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn manhattan(&self, other: &Point) -> usize {
        self.x.max(other.x) + self.y.max(other.y) - self.x.min(other.x) - self.y.min(other.y)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Creature {
    species: Species,
    hit_points: i16,
    attack_power: i16,
    position: Point,
    alive: bool,
}

impl Creature {
    fn new(species: Species, x: usize, y: usize) -> Self {
        Creature {
            species,
            hit_points: 200,
            attack_power: 3,
            position: Point { x, y },
            alive: true,
        }
    }
}

impl Ord for Creature {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.x.cmp(&other.position.x).then(self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for Creature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.species {
            Species::Goblin => write!(f, "{}", 'G'),
            Species::Elf => write!(f, "{}", 'E'),
        }
    }
}

#[derive(Hash)]
struct Wall {
    position: Point,
}

impl PartialEq for Wall {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Wall {}

struct Cave {
    walls: HashSet<Wall>,
    creatures: Vec<Creature>,
}

fn get_cave() -> Cave {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cave: Cave = Cave {
        walls: HashSet::new(),
        creatures: Vec::new(),
    };

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    cave.walls.insert(Wall {
                        position: Point { x, y },
                    });
                    continue;
                }
                'G' => cave.creatures.push(Creature::new(Species::Goblin, x, y)),
                'E' => cave.creatures.push(Creature::new(Species::Elf, x, y)),
                '.' => (),
                _ => panic!("Invalid character: {}", ch),
            };
        }
    }
    cave.creatures.sort_by_key(|c| c.position);

    cave
}

fn get_step(
    current: &Creature,
    walls: &HashSet<Point>,
    creatures: HashSet<Point>,
    enemies: HashSet<Point>,
) -> Option<Point> {
    let mut todo = BinaryHeap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();

    todo.push(Reverse((0, current.position)));

    while let Some(Reverse((step, position))) = todo.pop() {
        let possibilities = [
            Point {
                x: position.x,
                y: position.y - 1,
            },
            Point {
                x: position.x + 1,
                y: position.y,
            },
            Point {
                x: position.x,
                y: position.y + 1,
            },
            Point {
                x: position.x - 1,
                y: position.y,
            },
        ];

        for possibility in &possibilities {
            if !prev.contains_key(possibility) && !walls.contains(possibility) {
                if enemies.contains(possibility) {
                    return prev.remove(&position);
                } else if !creatures.contains(possibility) {
                    let prev_step = *prev.get(&position).unwrap_or(possibility);
                    prev.insert(*possibility, prev_step);
                    todo.push(Reverse((step + 1, *possibility)));
                }
            }
        }
    }
    None
}

fn get_attack(attacker: &Creature, enemies: Vec<&Creature>) -> Option<usize> {
    let victim = enemies
        .iter()
        .enumerate()
        .filter(|&(_, c)| c.alive && c.species != attacker.species)
        .filter(|&(_, &e)| attacker.position.manhattan(&e.position) == 1)
        .min_by(|&(_, &a), &(_, &e)| {
            a.hit_points
                .cmp(&e.hit_points)
                .then(a.position.cmp(&e.position))
        });
    match victim {
        Some((position, _)) => Some(position),
        None => None,
    }
}

fn print_cave(cave: &Cave) {
    let wall_positions: HashSet<Point> = HashSet::from_iter(
        cave.walls
            .iter()
            .map(|w| w.position)
            .collect::<Vec<Point>>(),
    );
    let creature_positions = &cave
        .creatures
        .iter()
        .filter(|c| c.alive)
        .map(|c| (c.position, *c))
        .collect::<HashMap<Point, Creature>>();

    let width = cave
        .walls
        .iter()
        .max_by(|a, b| a.position.x.cmp(&b.position.x))
        .unwrap()
        .position
        .x;
    let height = cave
        .walls
        .iter()
        .max_by(|a, b| a.position.y.cmp(&b.position.y))
        .unwrap()
        .position
        .y;

    for i in 0..height {
        for j in 0..width {
            let position = Point { x: j, y: i };
            if wall_positions.contains(&position) {
                print!("#");
            } else if creature_positions.contains_key(&position) {
                print!("{}", creature_positions.get(&position).unwrap());
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut cave = get_cave();
    let walls: HashSet<Point> = HashSet::from_iter(
        cave.walls
            .iter()
            .map(|w| w.position)
            .collect::<Vec<Point>>(),
    );
    let mut rounds = 0;
    loop {
        for i in 0..cave.creatures.len() {
            cave.creatures.sort();
            if !cave.creatures[i].alive {
                continue;
            }
            let creatures = cave.creatures.iter().collect::<Vec<&Creature>>();
            // If the unit is already in range of a target, it does not move, but continues its
            // turn with an attack.
            if let Some(enemy) = get_attack(&cave.creatures[i], creatures) {
                cave.creatures[enemy].hit_points -= cave.creatures[i].attack_power;
                if cave.creatures[enemy].hit_points < 1 {
                    cave.creatures[enemy].alive = false;
                }
            } else {
                // Otherwise, since it is not in range of a target, it moves.
                let creatures = HashSet::from_iter(
                    cave.creatures
                        .iter()
                        .filter(|c| c.alive && c.position != cave.creatures[i].position)
                        .map(|c| c.position)
                        .collect::<Vec<Point>>(),
                );
                let enemy_positions = HashSet::from_iter(
                    cave.creatures
                        .iter()
                        .filter(|c| c.alive && c.species != cave.creatures[i].species)
                        .map(|c| c.position),
                );
                if let Some(step) = get_step(&cave.creatures[i], &walls, creatures, enemy_positions)
                {
                    cave.creatures[i].position = step;
                }
                // After moving…the unit attacks.
                let creatures = cave.creatures.iter().collect::<Vec<&Creature>>();
                if let Some(enemy) = get_attack(&cave.creatures[i], creatures) {
                    cave.creatures[enemy].hit_points -= cave.creatures[i].attack_power;
                    if cave.creatures[enemy].hit_points < 1 {
                        cave.creatures[enemy].alive = false;
                    }
                }
            }
            // If no targets remain, combat ends.
            let hit_points: i16 = cave.creatures.iter().map(|c| c.hit_points).sum();
            let species: HashSet<Species> = HashSet::from_iter(
                cave.creatures
                    .iter()
                    .filter(|c| c.alive)
                    .map(|c| c.species)
                    .collect::<Vec<Species>>(),
            );
            if species.len() == 1 {
                //                print_cave(&cave);
                panic!("{}", hit_points as usize * (rounds) as usize);
            }
        }
        //        thread::sleep(time::Duration::from_millis(100));
        rounds += 1;
        //        print_cave(&cave);
    }
}
