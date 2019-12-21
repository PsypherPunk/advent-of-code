use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Intcode {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
    inputs: Vec<isize>,
}

struct Droid {
    intcode: Intcode,
    position: (isize, isize),
}

impl Droid {
    fn new(input: &str) -> Self {
        Droid {
            intcode: Intcode::new(input),
            position: (0, 0),
        }
    }
}

impl Intcode {
    fn new(input: &str) -> Intcode {
        let mut opcode = parse_opcodes(input);
        let mut extension = vec![0; 10000];
        opcode.append(&mut extension);
        Intcode {
            opcode,
            position: 0,
            relative_base: 0,
            inputs: Vec::new(),
        }
    }

    fn get_param1(&mut self) -> isize {
        match (self.opcode[self.position] / 100) % 10 {
            0 => self.opcode[self.opcode[self.position + 1] as usize],
            1 => self.opcode[self.position + 1],
            2 => self.opcode[(self.relative_base + self.opcode[self.position + 1]) as usize],
            _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
        }
    }

    fn get_param2(&mut self) -> isize {
        match (self.opcode[self.position] / 1000) % 10 {
            0 => self.opcode[self.opcode[self.position + 2] as usize],
            1 => self.opcode[self.position + 2],
            2 => self.opcode[(self.relative_base + self.opcode[self.position + 2]) as usize],
            _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
        }
    }

    fn get_param3(&mut self) -> usize {
        match (self.opcode[self.position] / 10000) % 10 {
            0 => self.opcode[self.position + 3] as usize,
            2 => (self.relative_base + self.opcode[self.position + 3]) as usize,
            _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
        }
    }

    fn run(&mut self) -> Option<isize> {
        loop {
            let opcode = self.opcode[self.position] % 100;
            match opcode {
                1 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output = p1 + p2;
                    let output_position = self.get_param3();
                    self.opcode[output_position] = output;
                    self.position += 4;
                }
                2 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output = p1 * p2;
                    let output_position = self.get_param3();
                    self.opcode[output_position] = output;
                    self.position += 4;
                }
                3 => {
                    let output_position = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.position + 1] as usize,
                        2 => (self.relative_base + self.opcode[self.position + 1]) as usize,
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let i = match self.inputs.pop() {
                        Some(i) => i as usize,
                        None => panic!("Attempt to read empty inputs!"),
                    };
                    self.opcode[output_position as usize] = i as isize;
                    self.position += 2;
                }
                4 => {
                    let p1 = self.get_param1();
                    self.position += 2;
                    return Some(p1);
                }
                5 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    if p1 != 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                6 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    if p1 == 0 {
                        self.position = p2 as usize;
                    } else {
                        self.position += 3;
                    }
                }
                7 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output_position = self.get_param3();
                    if p1 < p2 {
                        self.opcode[output_position as usize] = 1;
                    } else {
                        self.opcode[output_position as usize] = 0;
                    }
                    self.position += 4;
                }
                8 => {
                    let p1 = self.get_param1();
                    let p2 = self.get_param2();
                    let output_position = self.get_param3();
                    if p1 == p2 {
                        self.opcode[output_position] = 1;
                    } else {
                        self.opcode[output_position] = 0;
                    }
                    self.position += 4;
                }
                9 => {
                    let p1 = self.get_param1();
                    self.relative_base += p1;
                    self.position += 2;
                }
                99 => break,
                _ => panic!(
                    "Invalid self.opcode at position {} for {:?}",
                    self.position, self.opcode
                ),
            };
        }
        None
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

fn parse_opcodes(initial_state: &str) -> Vec<isize> {
    initial_state
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn map_shield(droid: &mut Droid) -> Vec<Vec<char>> {
    let mut shield = Vec::new();
    let mut row = Vec::new();

    while let Some(c) = droid.intcode.run() {
        match char::from(c as u8) {
            '\n' => {
                if row.len() > 0 {
                    shield.push(row);
                    row = Vec::new();
                }
            }
            x => row.push(x),
        };
    }
    shield
}

fn locate_intersections(shield: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();

    for (y, row) in shield.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if x > 0 && y > 0 && x < (row.len() - 1) && y < (shield.len() - 1) {
                if shield[y][x] == '#'
                    && shield[y - 1][x] == '#'
                    && shield[y][x + 1] == '#'
                    && shield[y + 1][x] == '#'
                    && shield[y][x - 1] == '#'
                {
                    intersections.push((x, y));
                }
            }
        }
    }
    intersections
}

fn main() {
    let input = read_input();
    let mut droid = Droid::new(&input.trim());

    let shield = map_shield(&mut droid);
    println!("{:?}", shield);
    let intersections = locate_intersections(&shield);
    let sum_alignment_parameters: usize = intersections.iter().map(|(x, y)| *x * *y).sum();
    println!(
        "What is the sum of the alignment parameters for the scaffold intersections? {}",
        sum_alignment_parameters
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let shield = r#"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^.."#
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let intersections = locate_intersections(&shield);
        let sum_alignment_parameters: usize = intersections.iter().map(|(x, y)| *x * *y).sum();
        assert_eq!(sum_alignment_parameters, 76);
    }
}
