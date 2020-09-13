use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Debug)]
struct Intcode {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
    inputs: Vec<isize>,
}

struct Droid {
    intcode: Intcode,
    map: HashMap<(isize, isize), isize>,
    oxygen: Option<(isize, isize)>,
}

fn get_step(status: usize, current: &(isize, isize)) -> (isize, isize) {
    match status {
        1 => (current.0, current.1 - 1),
        2 => (current.0, current.1 + 1),
        3 => (current.0 - 1, current.1),
        4 => (current.0 + 1, current.1),
        _ => panic!("Invalid status code: {}", status),
    }
}

fn step_back(status: usize) -> usize {
    match status {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("Invalid status code: {}", status),
    }
}

impl Droid {
    fn new(input: &str) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Droid {
            intcode: Intcode::new(input),
            map: HashMap::new(),
            oxygen: None,
        }
    }

    fn map_section(&mut self) {
        let mut backtrace = Vec::new();
        let mut section = HashMap::new();

        let mut current = (0, 0);
        section.insert(current, 1);
        while let Some(direction) = (1..=4)
            .find(|command| section.get(&get_step(*command, &current)).is_none())
            .or_else(|| backtrace.pop())
        {
            let next_pos = get_step(direction, &current);
            self.intcode.inputs.push(direction as isize);
            let status = self.intcode.run().unwrap();
            let unvisited = section.insert(next_pos, status).is_none();
            match status {
                0 => {},
                1 => {
                    if unvisited {
                        backtrace.push(step_back(direction))
                    };
                    current = next_pos;
                },
                2 => {
                    self.oxygen = Some(next_pos);
                    if unvisited {
                        backtrace.push(step_back(direction))
                    };
                    current = next_pos;
                },
                _ => {
                    panic!("Well, that's weird.");
                }
            };
        }
        self.map = section;
    }

    fn get_distances(&self, origin: (isize, isize)) -> HashMap<(isize, isize), isize> {
        let mut distances = HashMap::new();
        distances.insert(origin, 0);

        let mut work = VecDeque::new();
        work.push_back(origin);

        while let Some(step) = work.pop_front() {
            let options: Vec<(isize, isize)> = (1..=4)
                .map(|command| get_step(command, &step))
                .filter(|s| distances.get(s).is_none() && *self.map.get(s).unwrap_or(&0) > 0)
                .collect();
            let distance = 1 + *distances.get(&step).unwrap();
            for option in options {
                distances.insert(option, distance);
                work.push_back(option);
            }
        }
        distances
    }
}

impl Intcode {
    fn new(input: &str) -> Intcode {
        let mut opcode = parse_opcodes(input);
        let mut extension = vec![0; 1000];
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

fn parse_opcodes(initial_state: &str) -> Vec<isize> {
    initial_state
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut droid = Droid::new(&input);
    droid.map_section();
    let distances = droid.get_distances((0, 0));
    println!(
        "What is the fewest number of movement commands…to the location of the oxygen system? {}",
        distances.get(&droid.oxygen.unwrap()).unwrap(),
    );

    let distances = droid.get_distances(droid.oxygen.unwrap());
    println!(
        "How many minutes will it take to fill with oxygen? {}",
        distances.values().max().unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
