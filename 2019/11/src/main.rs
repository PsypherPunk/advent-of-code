use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
struct Intcode {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
    inputs: Vec<isize>,
}

#[derive(Clone, Debug)]
struct Robot {
    location: (isize, isize),
    orientation: Orientation,
    squares: HashMap<(isize, isize), isize>,
    intcode: Intcode,
}

impl Robot {
    fn new(input: &str) -> Robot {
        Robot {
            location: (0, 0),
            orientation: Orientation::Up,
            squares: HashMap::new(),
            intcode: Intcode::new(input),
        }
    }

    fn advance(&mut self) {
        self.location = match self.orientation {
            Orientation::Up => (self.location.0, self.location.1 - 1),
            Orientation::Right => (self.location.0 + 1, self.location.1),
            Orientation::Down => (self.location.0, self.location.1 + 1),
            Orientation::Left => (self.location.0 - 1, self.location.1),
        };
    }

    fn rotate(&mut self, rotation: isize) {
        self.orientation = match self.orientation {
            Orientation::Up => {
                if rotation == 0 {
                    Orientation::Left
                } else {
                    Orientation::Right
                }
            }
            Orientation::Right => {
                if rotation == 0 {
                    Orientation::Up
                } else {
                    Orientation::Down
                }
            }
            Orientation::Down => {
                if rotation == 0 {
                    Orientation::Right
                } else {
                    Orientation::Left
                }
            }
            Orientation::Left => {
                if rotation == 0 {
                    Orientation::Down
                } else {
                    Orientation::Up
                }
            }
        };
    }

    fn render(&self) {
        let mut y_coords = self.squares.keys().map(|(_, y)| *y).collect::<Vec<isize>>();
        y_coords.sort_unstable();
        let mut x_coords = self.squares.keys().map(|(x, _)| *x).collect::<Vec<isize>>();
        x_coords.sort_unstable();
        for y in *y_coords.first().unwrap()..=*y_coords.last().unwrap() {
            for x in *x_coords.first().unwrap()..=*x_coords.last().unwrap() {
                let square = match self.squares.get(&(x, y)) {
                    Some(1) => '#',
                    _ => ' ',
                };
                print!("{}", square);
            }
            println!();
        }
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

#[allow(clippy::while_let_loop)]
fn main() {
    let input = read_input();

    let mut robot = Robot::new(&input);
    robot.intcode.inputs.push(0);
    loop {
        let colour = match robot.intcode.run() {
            Some(n) => n,
            None => break,
        };
        robot.squares.insert(robot.location, colour);
        let rotation = match robot.intcode.run() {
            Some(n) => n,
            None => break,
        };
        robot.rotate(rotation);
        robot.advance();
        let input = match robot.squares.get(&robot.location) {
            Some(c) => *c,
            None => 0,
        };
        robot.intcode.inputs.push(input);
    }
    println!(
        "How many panels does it paint at least once? {}",
        robot.squares.keys().len()
    );

    let mut robot = Robot::new(&input);
    robot.intcode.inputs.push(1);
    loop {
        let colour = match robot.intcode.run() {
            Some(n) => n,
            None => break,
        };
        robot.squares.insert(robot.location, colour);
        let rotation = match robot.intcode.run() {
            Some(n) => n,
            None => break,
        };
        robot.rotate(rotation);
        robot.advance();
        let input = match robot.squares.get(&robot.location) {
            Some(c) => *c,
            None => 0,
        };
        robot.intcode.inputs.push(input);
    }
    robot.render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
