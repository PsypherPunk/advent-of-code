use std::collections::VecDeque;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
enum State {
    Ready,
    NeedInput,
}

#[derive(Clone, Debug)]
struct Intcode {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
    inputs: VecDeque<isize>,
    state: State,
}

struct Computer {
    intcode: Intcode,
}

#[derive(Debug)]
struct Packet {
    destination: isize,
    x: isize,
    y: isize,
}

impl Packet {
    fn from_vec(instructions: Vec<isize>) -> Packet {
        Packet {
            destination: instructions[0],
            x: instructions[1],
            y: instructions[2],
        }
    }
}

impl Computer {
    fn new(input: &str, address: usize) -> Self {
        let mut computer = Computer {
            intcode: Intcode::new(input),
        };
        computer.intcode.inputs.push_back(address as isize);
        computer
    }

    fn run(&mut self) -> Option<Packet> {
        let mut instructions = Vec::with_capacity(3);

        while let Some(output) = self.intcode.run() {
            instructions.push(output);
            if instructions.len() == 3 {
                return Some(Packet::from_vec(instructions));
            }
        }
        None
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
            inputs: VecDeque::new(),
            state: State::Ready,
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
                    if self.inputs.is_empty() {
                        self.state = State::NeedInput;
                        return None;
                    }
                    let output_position = match (self.opcode[self.position] / 100) % 10 {
                        0 => self.opcode[self.position + 1] as usize,
                        2 => (self.relative_base + self.opcode[self.position + 1]) as usize,
                        _ => panic!("Invalid mode at position {}", self.opcode[self.position]),
                    };
                    let i = match self.inputs.pop_front() {
                        Some(i) => i,
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

fn network(mut computers: Vec<Computer>) -> isize {
    let mut packets: VecDeque<Packet> = VecDeque::new();
    loop {
        for computer in computers.iter_mut() {
            if let Some(packet) = computer.run() {
                packets.push_back(packet);
            }
        }
        while !packets.is_empty() {
            let packet = packets.pop_front().unwrap();
            if packet.destination == 255 {
                return packet.y;
            }
            computers[packet.destination as usize]
                .intcode
                .inputs
                .push_back(packet.x);
            computers[packet.destination as usize]
                .intcode
                .inputs
                .push_back(packet.y);
            computers[packet.destination as usize].intcode.state = State::Ready;
        }
        for computer in computers.iter_mut() {
            if computer.intcode.state == State::NeedInput {
                computer.intcode.inputs.push_back(-1);
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt.");

    let computers = (0..50)
        .map(|address| Computer::new(&input, address))
        .collect::<Vec<Computer>>();

    println!(
        "What is the Y value of the first packet sent to address 255? {}",
        network(computers),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
