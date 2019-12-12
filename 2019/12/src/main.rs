use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
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

fn simulate(moons: &mut Vec<Moon>, steps: usize) {
    for _ in 0..steps {
        for i in 0..moons.len() {
            for j in i..moons.len() {
                let gravity = moons[i].get_gravitational_pull(&moons[j]);
                moons[i].obey_gravity(&gravity);
                moons[j].obey_gravity(&[-gravity[0], -gravity[1], -gravity[2]]);
            }
            moons[i].apply_velocity();
        }
    }
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
    println!("What is the total energy in the system after simulating the moons given in your scan for 1000 steps? {}", total_system_energy);
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
}
