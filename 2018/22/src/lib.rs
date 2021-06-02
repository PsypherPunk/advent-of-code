use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Rocky,
    Narrow,
    Wet,
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
    #[allow(dead_code)]
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
                    .map(|x| self.get_erosion_level(&(x, y)) % 3)
                    .sum::<usize>()
            })
            .sum()
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
}
