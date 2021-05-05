use std::collections::HashMap;
use std::str::FromStr;

use num::complex::Complex;

pub struct RoutingDiagram {
    lines: HashMap<Complex<isize>, char>,
}

impl FromStr for RoutingDiagram {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Complex::new(x as isize, y as isize), c))
            })
            .filter(|(_, c)| *c != ' ')
            .collect();

        Ok(Self { lines })
    }
}

impl RoutingDiagram {
    fn get_start(&self) -> Complex<isize> {
        let (start, _) = self
            .lines
            .iter()
            .find(|(position, &c)| position.im == 0 && c == '|')
            .unwrap();

        *start
    }

    pub fn get_letters(&self) -> String {
        let mut current = self.get_start();
        let mut direction = Complex::new(0, 1);
        let mut letters = Vec::new();

        let left = Complex::i().powi(-1);
        let right = Complex::i().powi(1);

        while let Some(c) = self.lines.get(&current) {
            match c {
                'A'..='Z' => letters.push(c),
                '+' => {
                    if self.lines.contains_key(&(current + (direction * right))) {
                        direction *= right;
                    } else {
                        direction *= left;
                    }
                }
                _ => {}
            }
            current += direction;
        }

        letters.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+"#;

    #[test]
    fn test_get_start() {
        let routing_diagram = RoutingDiagram::from_str(&INPUT).unwrap();

        assert_eq!(Complex::new(5, 0), routing_diagram.get_start());
    }

    #[test]
    fn test_part_one() {
        let routing_diagram = RoutingDiagram::from_str(&INPUT).unwrap();

        assert_eq!(String::from("ABCDEF"), routing_diagram.get_letters());
    }
}
