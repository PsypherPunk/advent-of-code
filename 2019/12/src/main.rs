use std::cmp::{max, min};
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Moon {
    position: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    //    fn new(x: isize, y: isize, z: isize) -> Self {
    //        Moon {
    //            position: [x, y, z],
    //            velocity: [0, 0, 0],
    //        }
    //    }

    fn from_string(input: &str) -> Vec<Moon> {
        let moons = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        moons
            .captures_iter(input)
            .map(|capture| Moon {
                position: [
                    capture[1].parse::<isize>().unwrap(),
                    capture[2].parse::<isize>().unwrap(),
                    capture[3].parse::<isize>().unwrap(),
                ],
                velocity: [0, 0, 0],
            })
            .collect::<Vec<Moon>>()
    }

    fn get_gravitational_pull(&self, other: &Moon) -> [isize; 3] {
        let gravity: Vec<isize> = other
            .position
            .iter()
            .zip(self.position.iter())
            .map(|(o, s)| (o - s).signum())
            .collect();
        [gravity[0], gravity[1], gravity[2]]
    }

    fn obey_gravity(&mut self, gravity: &[isize; 3]) {
        self.velocity
            .iter_mut()
            .zip(gravity.iter())
            .for_each(|(velocity, gravity)| *velocity += *gravity);
    }

    fn apply_velocity(&mut self) {
        self.position
            .iter_mut()
            .zip(self.velocity.iter())
            .for_each(|(position, velocity)| *position += *velocity);
    }

    fn get_total_energy(&self) -> isize {
        let potential_energy =
            self.position[0].abs() + self.position[1].abs() + self.position[2].abs();
        let kinetic_energy =
            self.velocity[0].abs() + self.velocity[1].abs() + self.velocity[2].abs();

        potential_energy * kinetic_energy
    }
}

fn simulate_moons(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in i..moons.len() {
            let gravity = moons[i].get_gravitational_pull(&moons[j]);
            moons[i].obey_gravity(&gravity);
            moons[j].obey_gravity(&[-gravity[0], -gravity[1], -gravity[2]]);
        }
        moons[i].apply_velocity();
    }
}

fn simulate(moons: &mut Vec<Moon>, steps: usize) {
    for _ in 0..steps {
        simulate_moons(moons);
    }
}

fn get_matching_axes(moons: &[Moon], initial: &[Moon]) -> [bool; 3] {
    let mut matches = [true, true, true];
    moons
        .iter()
        .zip(initial.iter())
        .for_each(|(current, original)| {
            for (i, matching_axis) in matches.iter_mut().enumerate() {
                *matching_axis = *matching_axis
                    && (current.position[i] == original.position[i]
                        && current.velocity[i] == original.velocity[i]);
            }
        });
    matches
}

/// https://rosettacode.org/wiki/Least_common_multiple#Rust
fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn simulate_universe(moons: &mut Vec<Moon>) -> usize {
    let initial = moons.clone();
    let mut cycles: [Option<usize>; 3] = [None, None, None];
    let mut steps: usize = 0;

    while !cycles.iter().all(|cycle| cycle.is_some()) {
        simulate_moons(moons);
        steps += 1;

        for (dimension, is_match) in get_matching_axes(moons, &initial).iter().enumerate() {
            if cycles[dimension].is_none() && *is_match {
                cycles[dimension] = Some(steps);
            }
        }
    }

    cycles.iter().fold(1, |acc, cycle| lcm(acc, cycle.unwrap()))
}

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn main() {
    let input = read_input();
    let mut moons = Moon::from_string(&input);
    simulate(&mut moons, 1000);
    let total_system_energy: isize = moons.iter().map(|moon| moon.get_total_energy()).sum();
    println!(
        "What is the total energy in the system…? {}",
        total_system_energy,
    );

    let mut moons = Moon::from_string(&input);
    let steps = simulate_universe(&mut moons);
    println!("How many steps does it take…? {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        let input = String::from(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        );
        let mut moons = Moon::from_string(&input);
        simulate(&mut moons, 10);
        let total_system_energy: isize = moons.iter().map(|moon| moon.get_total_energy()).sum();
        assert_eq!(moons[0].position, [2, 1, -3]);
        assert_eq!(moons[1].position, [1, -8, 0]);
        assert_eq!(moons[2].position, [3, -6, 1]);
        assert_eq!(moons[3].position, [2, 0, 4]);
        assert_eq!(moons[0].velocity, [-3, -2, 1]);
        assert_eq!(moons[1].velocity, [-1, 1, 3]);
        assert_eq!(moons[2].velocity, [3, 2, -3]);
        assert_eq!(moons[3].velocity, [1, -1, -1]);
        assert_eq!(total_system_energy, 179);
    }

    #[test]
    fn test_2772() {
        let input = String::from(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        );
        let mut initial = Moon::from_string(&input);
        let moons = initial.clone();
        simulate(&mut initial, 2772);
        assert_eq!(initial, moons);
    }
}
