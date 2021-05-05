use std::collections::HashMap;
use std::str::FromStr;

struct Particle {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
    acceleratation: (isize, isize, isize),
}

impl Particle {
    fn tick(&mut self) {
        self.velocity.0 += self.acceleratation.0;
        self.velocity.1 += self.acceleratation.1;
        self.velocity.2 += self.acceleratation.2;
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn get_manhattan_distance(&self) -> isize {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
}

pub struct Buffer {
    particles: Vec<Particle>,
}

impl FromStr for Buffer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let particles = s
            .trim()
            .lines()
            .map(|line| {
                let (p, v, a) = match line.trim().split(", ").collect::<Vec<_>>()[..] {
                    [p, v, a] => {
                        let p = match p[3..(p.len() - 1)].split(',').collect::<Vec<_>>()[..] {
                            [px, py, pz] => (
                                px.parse().unwrap(),
                                py.parse().unwrap(),
                                pz.parse().unwrap(),
                            ),
                            _ => panic!(r#"¯\_(ツ)_/¯"#),
                        };
                        let v = match v[3..(v.len() - 1)].split(',').collect::<Vec<_>>()[..] {
                            [vx, vy, vz] => (
                                vx.parse().unwrap(),
                                vy.parse().unwrap(),
                                vz.parse().unwrap(),
                            ),
                            _ => panic!(r#"¯\_(ツ)_/¯"#),
                        };
                        let a = match a[3..(a.len() - 1)].split(',').collect::<Vec<_>>()[..] {
                            [ax, ay, az] => (
                                ax.parse().unwrap(),
                                ay.parse().unwrap(),
                                az.parse().unwrap(),
                            ),
                            _ => panic!(r#"¯\_(ツ)_/¯"#),
                        };
                        (p, v, a)
                    }
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                Particle {
                    position: p,
                    velocity: v,
                    acceleratation: a,
                }
            })
            .collect();

        Ok(Self { particles })
    }
}

impl Buffer {
    fn tick(&mut self) {
        self.particles.iter_mut().for_each(|particle| {
            particle.tick();
        });
    }

    fn resolve_collisions(&mut self) {
        let mut counts = HashMap::new();
        self.particles.iter().for_each(|particle| {
            *counts.entry(&particle.position).or_insert(0) += 1;
        });

        let collisions = counts
            .iter()
            .filter(|(_, &count)| count > 1)
            .map(|(&position, _)| *position)
            .collect::<Vec<_>>();

        self.particles
            .retain(|particle| !collisions.contains(&particle.position));
    }

    pub fn get_closest_to_zero(&mut self) -> usize {
        let mut manhattan_distances = vec![0; self.particles.len()];

        for _ in 0..5_000 {
            self.particles
                .iter_mut()
                .enumerate()
                .for_each(|(n, particle)| {
                    manhattan_distances[n] += particle.get_manhattan_distance();
                });
            self.tick();
        }

        let min = manhattan_distances
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap();

        min.0
    }

    pub fn get_particle_count(&mut self) -> usize {
        for _ in 0..5_000 {
            self.resolve_collisions();
            self.tick();
        }

        self.particles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"#;
        let mut buffer = Buffer::from_str(&input).unwrap();

        assert_eq!(0, buffer.get_closest_to_zero());
    }

    #[test]
    fn test_part_two() {
        let input = r#"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"#;
        let mut buffer = Buffer::from_str(&input).unwrap();

        assert_eq!(1, buffer.get_particle_count());
    }
}
