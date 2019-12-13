use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

use termion::{clear, cursor};

#[derive(Clone, Debug)]
struct Intcode {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
    inputs: Vec<isize>,
}

#[derive(Clone, Debug)]
struct Cabinet {
    intcode: Intcode,
    screen: HashMap<(isize, isize), isize>,
    score: isize,
}

impl Cabinet {
    fn new(input: &str) -> Self {
        Cabinet {
            intcode: Intcode::new(input),
            screen: HashMap::new(),
            score: 0,
        }
    }

    fn render(&self) {
        let width = self.screen.keys().map(|(x, _)| *x).max().unwrap();
        let height = self.screen.keys().map(|(_, y)| *y).max().unwrap();
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        for y in 0..=height {
            for x in 0..=width {
                let tile = match self.screen.get(&(x as isize, y as isize)) {
                    Some(t) => match t {
                        0 => " ",
                        1 => "+",
                        2 => "#",
                        3 => "=",
                        4 => "@",
                        _ => panic!("Oops!"),
                    },
                    None => " ",
                };
                print!("{}", tile);
            }
            println!();
        }
        println!("Score: {}", self.score);
        thread::sleep(time::Duration::from_millis(10));
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

fn main() {
    let input = read_input();

    let mut cabinet = Cabinet::new(&input);
    'outer: loop {
        let mut tile: [isize; 3] = [0, 0, 0];
        for i in 0..=2 {
            tile[i] = match cabinet.intcode.run() {
                Some(n) => n,
                None => break 'outer,
            };
        }
        cabinet.screen.insert((tile[0], tile[1]), tile[2]);
    }
    let blocks = cabinet
        .screen
        .values()
        .filter(|tile| *tile == &2)
        .collect::<Vec<&isize>>();
    println!(
        "How many block tiles are on the screen when the game exits? {}",
        blocks.len()
    );

    let mut cabinet = Cabinet::new(&input);
    cabinet.intcode.opcode[0] = 2;
    'outer: loop {
        let mut tile: [isize; 3] = [0, 0, 0];
        for i in 0..=2 {
            tile[i] = match cabinet.intcode.run() {
                Some(n) => n,
                None => break 'outer,
            };
        }
        if tile[0] == -1 && tile[1] == 0 {
            cabinet.score = tile[2];
        } else {
            cabinet.screen.insert((tile[0], tile[1]), tile[2]);
        }
        let ball = cabinet.screen.iter().find(|(_, tile)| *tile == &4);
        let paddle = cabinet.screen.iter().find(|(_, tile)| *tile == &3);
        if let Some(ball) = ball {
            if let Some(paddle) = paddle {
                match (ball.0).0.cmp(&(paddle.0).0) {
                    Ordering::Less => cabinet.intcode.inputs.push(-1),
                    Ordering::Greater => cabinet.intcode.inputs.push(1),
                    Ordering::Equal => cabinet.intcode.inputs.push(0),
                }
            }
        }
        cabinet.render();
    }
    println!(
        "What is your score after the last block is broken? {}",
        cabinet.score
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
