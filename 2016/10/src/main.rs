use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum Target {
    Bot,
    Output,
}

#[derive(Clone, Debug)]
struct Instruction {
    high: bool,
    target: Target,
    id: usize,
}

#[derive(Debug)]
struct Bot {
    id: usize,
    chips: HashMap<bool, usize>,
}

impl Bot {
    fn new(id: usize) -> Self {
        Self {
            id,
            chips: HashMap::new(),
        }
    }

    fn is_ready(&self) -> bool {
        self.chips.len() == 2
    }

    fn receive_chip(&mut self, new_chip: usize) {
        let mut chips = self
            .chips
            .iter()
            .map(|(_, chip)| *chip)
            .collect::<Vec<_>>();
        chips.push(new_chip);
        chips.sort();
        if chips.len() == 2 && *chips.get(0).unwrap() == 17 && *chips.get(1).unwrap() == 61 {
            println!(
                "what is the…bot that is…comparing value-61 microchips with value-17 microchips? {}",
                self.id,
            );
        }
        self.chips = chips
            .iter()
            .enumerate()
            .map(|(index, chip)| (index == 1, *chip))
            .collect::<HashMap<bool, usize>>();
    }

    fn take_chip(&mut self, high: bool) -> usize {
        self.chips.remove(&high).unwrap()
    }
}

#[derive(Debug)]
struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, Vec<usize>>,
    instructions: HashMap<usize, Vec<Instruction>>,
}

impl Factory {
    fn new(input: &str) -> Self {
        let mut bots = HashMap::new();
        let mut outputs = HashMap::new();
        let mut instructions = HashMap::new();

        input.trim().lines().for_each(|line| {
            let words = line.split_whitespace().collect::<Vec<_>>();
            match words[0] {
                "value" => {
                    let chip_id = words[1].parse().unwrap();
                    let bot_id = words[5].parse().unwrap();

                    let bot = bots.entry(bot_id).or_insert_with(|| Bot::new(bot_id));
                    bot.receive_chip(chip_id);
                }
                "bot" => {
                    let bot_id = words[1].parse().unwrap();
                    let low_target_id = words[6].parse().unwrap();
                    let high_target_id = words[11].parse().unwrap();

                    bots.entry(bot_id).or_insert_with(|| Bot::new(bot_id));
                    let bot_instructions = instructions.entry(bot_id).or_insert_with(Vec::new);
                    bot_instructions.push(Instruction {
                        high: false,
                        target: match words[5] {
                            "output" => {
                                outputs.entry(low_target_id).or_insert_with(Vec::new);
                                Target::Output
                            }
                            "bot" => Target::Bot,
                            _ => panic!("Invalid target: {}", words[5]),
                        },
                        id: low_target_id,
                    });
                    bot_instructions.push(Instruction {
                        high: true,
                        target: match words[10] {
                            "output" => {
                                outputs.entry(high_target_id).or_insert_with(Vec::new);
                                Target::Output
                            }
                            "bot" => Target::Bot,
                            _ => panic!("Invalid target: {}", words[5]),
                        },
                        id: high_target_id,
                    });
                }
                _ => panic!("Invalid instruction: {}", line),
            }
        });

        Self {
            bots,
            outputs,
            instructions,
        }
    }

    fn perform_instructions(&mut self) {
        for (id, bot_instructions) in self.instructions.iter() {
            let bot = self.bots.get(&id).unwrap();
            if !bot.is_ready() {
                continue;
            }
            for instruction in bot_instructions {
                let bot = self.bots.get_mut(&id).unwrap();
                let chip = bot.take_chip(instruction.high);
                match instruction.target {
                    Target::Bot => {
                        let target = self.bots.get_mut(&instruction.id).unwrap();
                        target.receive_chip(chip);
                    }
                    Target::Output => {
                        let output = self.outputs.get_mut(&instruction.id).unwrap();
                        output.push(chip);
                    }
                }
            }
        }
    }

    fn zoom(&mut self) {
        while self.bots.values().any(|bot| bot.is_ready()) {
            self.perform_instructions();
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut factory = Factory::new(&input);
    factory.zoom();

    let zero = *factory.outputs.get(&0).unwrap().get(0).unwrap();
    let one = *factory.outputs.get(&1).unwrap().get(0).unwrap();
    let two = *factory.outputs.get(&2).unwrap().get(0).unwrap();
    println!(
        "What do you get if you multiply together the values of…outputs 0, 1, and 2? {}",
        zero * one * two,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
"#;

        let mut factory = Factory::new(&input);
        factory.zoom();

        assert_eq!(true, factory.outputs.contains_key(&0));
        assert_eq!(1, factory.outputs.get(&0).unwrap().len());
        assert_eq!(5, *factory.outputs.get(&0).unwrap().get(0).unwrap());
    }
}
