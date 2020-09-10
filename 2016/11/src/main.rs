use std::collections::{BinaryHeap, HashSet};
use std::fs;

use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Component {
    Microchip(String),
    Generator(String),
}

impl Component {
    fn get_element(&self) -> &str {
        match self {
            Component::Generator(e) | Component::Microchip(e) => e,
        }
    }

    fn is_element(&self, element: &str) -> bool {
        match self {
            Component::Generator(e) | Component::Microchip(e) => e == element,
        }
    }

    fn is_generator(&self) -> bool {
        match self {
            Component::Generator(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct Floor(Vec<Component>);

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        let a: HashSet<_> = self.0.iter().collect();
        let b: HashSet<_> = other.0.iter().collect();

        a == b
    }
}

impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
    }
}

impl ToString for Floor {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|object| match object {
                Component::Generator(element) => {
                    format!("{}G", element.chars().next().unwrap().to_uppercase())
                }
                Component::Microchip(element) => {
                    format!("{}M", element.chars().next().unwrap().to_uppercase())
                }
            })
            .collect_vec()
            .join(" ")
    }
}

impl Floor {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn put_component(&mut self, component: Component) {
        self.0.push(component);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn has_generator_for(&self, element: &str) -> bool {
        self.0
            .iter()
            .any(|component| component.is_generator() && component.is_element(element))
    }

    fn is_valid(&self) -> bool {
        self.0
            .iter()
            .filter(|component| !component.is_generator())
            .all(|microchip| {
                self.has_generator_for(microchip.get_element())
                    || self
                        .0
                        .iter()
                        .find(|component| component.is_generator())
                        .is_none()
            })
    }

    /// Get valid combinations of items that can be moved.
    ///
    /// A valid combination must:
    ///
    /// * have a single component;
    /// * have all microchips;
    /// * have all generators;
    /// * have both a microchip and generator for the same element.
    fn get_combinations(&self) -> Vec<Vec<&Component>> {
        (1..=2)
            .rev()
            .flat_map(|count| self.0.iter().combinations(count).collect_vec())
            .filter(|combination| {
                combination.len() == 1
                    || combination.iter().all(|component| match component {
                        Component::Microchip(_) => true,
                        _ => false,
                    })
                    || combination.iter().all(|component| match component {
                        Component::Generator(_) => true,
                        _ => false,
                    })
                    || combination
                        .iter()
                        .map(|component| match component {
                            Component::Generator(e) | Component::Microchip(e) => e,
                        })
                        .collect::<HashSet<_>>()
                        .len()
                        == 1
            })
            .collect_vec()
    }

    fn put_components(&mut self, components: &[&Component]) {
        self.0
            .append(&mut components.iter().map(|&c| c.clone()).collect_vec());
    }

    fn take_components(&mut self, components: &[&Component]) {
        self.0.retain(|component| !components.contains(&component));
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Facility {
    floors: Vec<Floor>,
    elevator: usize,
    steps: usize,
    score: usize,
}

impl Ord for Facility {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Facility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Facility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let output = (0..=3)
            .rev()
            .map(|floor| {
                let objects = self.floors.get(floor).unwrap().to_string();

                let elevator = if self.elevator == floor { "E" } else { "." };
                format!("F{} {} {}", floor + 1, elevator, objects)
            })
            .collect_vec()
            .join("\n");

        write!(f, "{}", output)
    }
}

impl Facility {
    fn from_string(input: &str) -> Self {
        let floors = input
            .trim()
            .lines()
            .map(|line| {
                let words = line.split(&[' ', '-', ',', '.'][..]).collect::<Vec<_>>();
                let mut floor = Floor::new();

                for index in 0..words.len() {
                    match words[index] {
                        "generator" => {
                            floor.put_component(Component::Generator(words[index - 1].to_string()));
                        }
                        "compatible" => {
                            floor.put_component(Component::Microchip(words[index - 1].to_string()));
                        }
                        _ => {}
                    }
                }
                floor
            })
            .collect_vec();

        Self {
            floors,
            elevator: 0,
            steps: 0,
            score: 0,
        }
    }

    /// Calculate the weighting for a given state.
    ///
    /// Higher scores are granted for having more items on the higher floors
    /// and keeping the lower floors empty.
    ///
    /// An `adjustment` is passed in, either boosting states move 2 items
    /// upwards or single items downwards.
    fn score(&mut self, adjustment: usize) {
        let item_count_upper_floors: usize = self
            .floors
            .iter()
            .enumerate()
            .map(|(num, floor)| (num.pow(adjustment as u32) * floor.len()))
            .sum();

        let empty_lower_floors: usize = self
            .floors
            .iter()
            .enumerate()
            .filter(|(_, floor)| floor.is_empty())
            .map(|(num, _)| (self.floors.len() - num).pow(adjustment as u32))
            .sum();

        self.score =
            (empty_lower_floors * item_count_upper_floors).pow(adjustment as u32) / self.steps
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_valid())
    }

    fn get_next_steps(&self) -> Vec<Facility> {
        (0..=3)
            .filter(|&floor| (floor as isize - self.elevator as isize).abs() == 1)
            .flat_map(|next_floor_num| {
                let going_up = next_floor_num > self.elevator;

                if !going_up && self.is_empty_below(self.elevator) {
                    return vec![];
                }

                self.floors
                    .get(self.elevator)
                    .unwrap()
                    .get_combinations()
                    .iter()
                    .map(|combination| {
                        let mut facility = Self {
                            floors: self.floors.clone(),
                            elevator: next_floor_num,
                            steps: self.steps + 1,
                            score: self.score,
                        };
                        facility
                            .floors
                            .get_mut(next_floor_num)
                            .unwrap()
                            .put_components(combination);
                        facility
                            .floors
                            .get_mut(self.elevator)
                            .unwrap()
                            .take_components(combination);

                        facility.score(if going_up {
                            combination.len()
                        } else {
                            2 / combination.len()
                        });

                        if facility.is_valid() {
                            Some(facility)
                        } else {
                            None
                        }
                    })
                    .filter_map(|facility| facility)
                    .collect_vec()
            })
            .collect::<Vec<_>>()
    }

    fn is_empty_below(&self, floor: usize) -> bool {
        self.floors
            .iter()
            .rev()
            .skip(self.floors.len() - (floor + 1))
            .all(|floor| floor.is_empty())
    }

    fn is_ready(&self) -> bool {
        self.is_empty_below(2)
    }

    fn get_quickest_route(&self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut discovered = HashSet::new();

        heap.push(self.clone());
        discovered.insert(self.clone());
        println!("{}\n", self);

        while !heap.is_empty() {
            let step = heap.pop().unwrap();
            if step.is_ready() {
                println!("{}\n", step);
                return step.steps;
            }

            for next in step.get_next_steps().iter().filter(|step| step.is_valid()) {
                if !discovered.contains(&next) {
                    heap.push(next.clone());
                    discovered.insert(next.clone());
                }
            }
        }
        panic!("Could not find a solution.")
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let facility = Facility::from_string(&input);
    println!(
        "…what is the minimum number of steps required…? {}",
        facility.get_quickest_route(),
    );

    let mut facility = Facility::from_string(&input);
    let ground_floor = facility.floors.get_mut(0).unwrap();
    ground_floor.put_components(&[
        &Component::Generator("elerium".into()),
        &Component::Microchip("elerium".into()),
        &Component::Generator("dilithium".into()),
        &Component::Microchip("dilithium".into()),
    ]);
    println!(
        "What is the minimum number of steps…to bring all of the objects…to the fourth floor? {}",
        facility.get_quickest_route(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant."#;

        let facility = Facility::from_string(&input);

        assert_eq!(11, facility.get_quickest_route());
    }
}
