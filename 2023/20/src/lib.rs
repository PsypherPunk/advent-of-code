use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

enum Type<'a> {
    FlipFlop { off: bool },
    Conjunction { last_pulses: Vec<(&'a str, Pulse)> },
}

struct Module<'a> {
    r#type: Type<'a>,
    destinations: Vec<&'a str>,
}

fn from_str(input: &str) -> Result<(Vec<&str>, HashMap<&str, Module>), String> {
    let broadcaster_targets = input
        .lines()
        .find_map(|line| {
            line.strip_prefix("broadcaster -> ")
                .map(|targets| targets.split(", ").collect::<Vec<_>>())
        })
        .ok_or(format!("couldn't find broadcaster: {}", input))?;

    let mut rules = input
        .lines()
        .filter(|line| !line.starts_with("broadcaster"))
        .map(|line| {
            let (name, outputs) = line[1..].split_once(" -> ").ok_or(format!("bad line: {}", line))?;
            let outputs = outputs.split(", ").collect::<Vec<_>>();

            let kind = match line.chars().next() {
                Some('%') => Type::FlipFlop { off: true },
                Some('&') => Type::Conjunction {
                    last_pulses: vec![],
                },
                _ => unreachable!("invalid module kind"),
            };

            let module = Module {
                r#type: kind,
                destinations: outputs,
            };
            Ok((name, module))
        })
        .collect::<Result<HashMap<_, _>, String>>()?;

    let input_to_outputs = rules
        .iter()
        .map(|(name, module)| (*name, module.destinations.clone()))
        .collect::<Vec<_>>();

    for (name, outputs) in input_to_outputs {
        outputs
            .iter()
            .filter(|&s| *s != "output")
            .for_each(|output| {
                let Some(module) = rules.get_mut(output) else {
                    return;
                };
                if let Type::Conjunction { last_pulses } = &mut module.r#type {
                    last_pulses.push((name, Pulse::Low));
                }
            });
    }

    Ok((broadcaster_targets, rules))
}

fn pulsify<'a>(
    queue: &mut VecDeque<(&'a str, &'a str, Pulse)>,
    rules: &mut HashMap<&'a str, Module<'a>>,
    name: &'a str,
    sender: &'a str,
    pulse: Pulse,
) -> Result<(), String> {
    let Some(module) = rules.get_mut(name) else {
        return Ok(());
    };
    let pulse_type = match &mut module.r#type {
        Type::FlipFlop { off } => {
            if pulse == Pulse::High {
                return Ok(());
            }
            let pulse_type = if *off { Pulse::High } else { Pulse::Low };
            *off = !*off;
            pulse_type
        }
        Type::Conjunction { last_pulses } => {
            last_pulses
                .iter_mut()
                .find(|(input, _pulse)| input == &sender)
                .ok_or("cannot find conjunction".to_owned())?
                .1 = pulse;

            let all_high = last_pulses.iter().all(|(_, pulse)| *pulse == Pulse::High);
            if all_high {
                Pulse::Low
            } else {
                Pulse::High
            }
        }
    };
    queue.extend(
        module
            .destinations
            .iter()
            .map(|target| (*target, name, pulse_type)),
    );

    Ok(())
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let (broadcaster_targets, mut modules) = from_str(input)?;
    let mut low_pulses = 1000;
    let mut high_pulses = 0;
    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        queue.extend(
            broadcaster_targets
                .iter()
                .map(|target| (*target, "broadcaster", Pulse::Low)),
        );

        while let Some((module, sender, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            };
            pulsify(&mut queue, &mut modules, module, sender, pulse)?;
        }
    }

    Ok(low_pulses * high_pulses)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let (broadcaster_targets, mut rules) = from_str(input)?;

    let rx_sender = rules
        .iter()
        .find_map(|(name, module)| module.destinations.contains(&"rx").then_some(*name))
        .ok_or("couldn't find senders to rx".to_owned())?;

    let rx_sender_senders = {
        let module = &rules[rx_sender];
        let Type::Conjunction { last_pulses } = &module.r#type else {
            unreachable!();
        };
        last_pulses
            .iter()
            .map(|(input, _pulse)| *input)
            .collect::<Vec<_>>()
    };

    let mut button_pushes = rx_sender_senders.iter().map(|_| None).collect::<Vec<_>>();

    let mut pushes = 1;
    let mut queue = VecDeque::new();
    while button_pushes.iter().any(Option::is_none) {
        queue.extend(
            broadcaster_targets
                .iter()
                .map(|target| (*target, "broadcaster", Pulse::Low)),
        );

        while let Some((name, parent_name, pulse)) = queue.pop_front() {
            pulsify(&mut queue, &mut rules, name, parent_name, pulse)?;
            if name == rx_sender {
                let sender = rx_sender_senders
                    .iter()
                    .position(|input| input == &parent_name)
                    .ok_or("couldn't find rx sender".to_owned())?;

                if button_pushes[sender].is_none() && pulse == Pulse::High {
                    button_pushes[sender] = Some(pushes);
                }
            }
        }

        pushes += 1;
    }

    let fewest = button_pushes
        .into_iter()
        .flatten()
        .reduce(num_integer::lcm)
        .ok_or("couldn't find fewest".to_owned())?;

    Ok(fewest)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;
    const TWO: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(32000000), get_part_one(ONE));
        assert_eq!(Ok(11687500), get_part_one(TWO));
    }
}
