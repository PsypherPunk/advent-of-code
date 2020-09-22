use std::fs;

struct Disc {
    positions: usize,
    position: usize,
}

impl Disc {
    fn is_open_after(&self, seconds: usize) -> bool {
        (self.position + seconds) % self.positions == 0
    }
}

struct Sculpture {
    discs: Vec<Disc>,
}

impl Sculpture {
    fn from_str(input: &str) -> Self {
        let discs = input
            .trim()
            .lines()
            .map(|line| {
                let words = line.split_whitespace().collect::<Vec<_>>();
                let position = words[11].replace('.', "");

                Disc {
                    positions: words[3].parse().unwrap(),
                    position: position.parse().unwrap(),
                }
            })
            .collect();

        Self { discs }
    }

    fn get_capsule(&self) -> usize {
        let mut seconds = 0;
        loop {
            if self
                .discs
                .iter()
                .enumerate()
                .all(|(index, disc)| disc.is_open_after(seconds + index + 1))
            {
                return seconds;
            }
            seconds += 1
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let sculpture = Sculpture::from_str(&input);
    println!(
        "What is the first time you can press the button to get a capsule? {}",
        sculpture.get_capsule(),
    );

    let mut sculpture = Sculpture::from_str(&input);
    sculpture.discs.push(Disc {
        positions: 11,
        position: 0,
    });
    println!(
        "…what is the first time you can press the button to get another capsule? {}",
        sculpture.get_capsule(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."#;

        let sculpture = Sculpture::from_str(&input);

        assert_eq!(5, sculpture.get_capsule());
    }
}
