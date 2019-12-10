use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Amplifier {
    opcode: Vec<isize>,
    position: usize,
    relative_base: isize,
}

impl Amplifier {
    fn new(input: &String) -> Amplifier {
        let mut opcode = parse_opcodes(input);
        let mut extension = vec![0; 1000];
        opcode.append(&mut extension);
        Amplifier {
            opcode,
            position: 0,
            relative_base: 0,
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

    fn run(&mut self, mut phase_setting: Option<&i32>, signal: usize) -> Option<isize> {
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
                    let i = match phase_setting.take() {
                        Some(i) => *i as usize,
                        None => signal,
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

fn parse_opcodes(initial_state: &String) -> Vec<isize> {
    initial_state
        .trim()
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn main() {
    let input = read_input();
    let mut amplifier: Amplifier = Amplifier::new(&input);
    let boost = amplifier.run(None, 1);
    println!("What BOOST keycode does it produce? {}", boost.unwrap());

    let mut amplifier: Amplifier = Amplifier::new(&input);
    let distress_signal = amplifier.run(None, 2);
    println!(
        "What are the coordinates of the distress signal? {}",
        distress_signal.unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10912041100110011001008100161011006101099() {
        let intcode = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut amplifier: Amplifier = Amplifier::new(&intcode);
        let mut outputs: Vec<isize> = Vec::new();
        loop {
            let output = amplifier.run(None, 0).unwrap();
            outputs.push(output);
            if output == 99 {
                break;
            }
        }
        let output = outputs
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(output, intcode);
    }

    #[test]
    fn test_11023491519234915192747990() {
        let intcode = String::from("1102,34915192,34915192,7,4,7,99,0");
        let mut amplifier: Amplifier = Amplifier::new(&intcode);
        let output = amplifier.run(None, 0);
        assert_eq!(output.unwrap().to_string().len(), 16);
    }

    #[test]
    fn test_104112589990684262499() {
        let intcode = String::from("104,1125899906842624,99");
        let mut amplifier: Amplifier = Amplifier::new(&intcode);
        let output = amplifier.run(None, 0);
        assert_eq!(output.unwrap(), 1125899906842624);
    }
}
