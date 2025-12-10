use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug)]
struct FactoryMachine {
    light_diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

peg::parser! {
    pub grammar factory_machine() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule light_diagram() -> Vec<bool>
            = "[" states:($(("#" / ".")+) ) "]"
                {
                    states.chars().map(|c| c == '#')
                        .collect::<Vec<_>>()
                }

        rule button() -> Vec<usize>
            = "(" indicator_lights:integer() ** "," ")"
                { indicator_lights }

        rule buttons() -> Vec<Vec<usize>>
            = btns:button() ** _
                { btns }

        rule joltage_requirements() -> Vec<usize>
            = "{" reqs:integer() ** "," "}"
                { reqs }

        pub rule machine() -> FactoryMachine
            = light_diagram:light_diagram()
              _
              buttons:buttons()
              _
              joltage_requirements:joltage_requirements()
                {
                    FactoryMachine {
                        light_diagram,
                        buttons,
                        joltage_requirements
                    }
                }
    }
}

fn fewest_presses(factory_machine: &FactoryMachine) -> Option<usize> {
    let initial_lights = vec![false; factory_machine.light_diagram.len()];

    let mut seen: HashSet<Vec<bool>> = HashSet::new();
    seen.insert(initial_lights.clone());

    let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::new();
    queue.push_back((initial_lights, 0));

    while let Some((current_lights, presses)) = queue.pop_front() {
        if current_lights == factory_machine.light_diagram {
            return Some(presses);
        }

        for button in &factory_machine.buttons {
            let next_lights = current_lights
                .iter()
                .enumerate()
                .map(|(i, &b)| if button.contains(&i) { !b } else { b })
                .collect::<Vec<_>>();

            if seen.insert(next_lights.clone()) {
                queue.push_back((next_lights, presses + 1));
            }
        }
    }

    None
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let factory_machines = input
        .lines()
        .map(factory_machine::machine)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let fewest_presses = factory_machines
        .iter()
        .map(fewest_presses)
        .collect::<Option<Vec<_>>>()
        .ok_or("invalid manual")?
        .iter()
        .sum();

    Ok(fewest_presses)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(7), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
