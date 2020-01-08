use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
enum Step {
    Open,
    Key(isize),
    Door(isize),
}

#[derive(Debug)]
struct Vault {
    steps: HashMap<(isize, isize), Step>,
    keys_bitmask: isize,
    position: (isize, isize),
}

impl Vault {
    fn from_string(input: &str) -> Self {
        let mut steps: HashMap<(isize, isize), Step> = HashMap::new();
        let mut keys_bitmask = 0;

        let mut position: (isize, isize) = (-1, -1);

        for (y, line) in input.lines().enumerate() {
            for (x, step) in line.chars().enumerate() {
                let p = (x as isize, y as isize);
                match step {
                    'a'..='z' => {
                        let key_bitmask = step as isize - 'a' as isize;
                        steps.insert(p, Step::Key(key_bitmask));
                        keys_bitmask |= 1 << key_bitmask;
                    }
                    'A'..='Z' => {
                        steps.insert(p, Step::Door(step as isize - 'A' as isize));
                    }
                    '.' => {
                        steps.insert(p, Step::Open);
                    }
                    '@' => {
                        steps.insert(p, Step::Open);
                        position = p;
                    }
                    _ => {}
                }
            }
        }

        Vault {
            steps,
            keys_bitmask,
            position,
        }
    }

    fn find_keys(
        steps: &HashMap<(isize, isize), Step>,
        seen: &mut HashSet<(isize, isize)>,
        position: (isize, isize),
        keys_bitmask: isize,
    ) -> isize {
        seen.insert(position);

        let mut found = match steps.get(&position) {
            Some(Step::Key(key)) => (1 << *key),
            _ => 0,
        };

        for (dx, dy) in &U_R_D_L {
            let next = (position.0 + dx, position.1 + dy);
            if !seen.contains(&next) && steps.contains_key(&next) {
                found |= Vault::find_keys(steps, seen, next, keys_bitmask);
            }
        }

        keys_bitmask | found
    }

    fn separate_sections(&self) -> Vec<Vault> {
        let (ox, oy) = self.position;
        let mut steps = self.steps.clone();
        steps.remove(&self.position);

        for (dx, dy) in &U_R_D_L {
            steps.remove(&(ox + dx, oy + dy));
        }

        let mut vaults = Vec::new();

        for (dx, dy) in &[(-1, -1), (1, 1), (1, -1), (-1, 1)] {
            let position = (ox + dx, oy + dy);
            steps.insert(position, Step::Open);

            vaults.push(Vault {
                steps: steps.clone(),
                position,
                keys_bitmask: Vault::find_keys(&steps, &mut HashSet::new(), position, 0),
            });
        }

        vaults
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    missing_keys_bitmask: isize,
    position: (isize, isize),
}

const U_R_D_L: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl State {
    fn from_vault(vault: &Vault) -> Self {
        State {
            missing_keys_bitmask: vault.keys_bitmask,
            position: vault.position,
        }
    }

    fn next(&self, vault: &Vault) -> Vec<State> {
        U_R_D_L
            .iter()
            .filter_map(|(dx, dy)| {
                let next = (self.position.0 + *dx, self.position.1 + *dy);

                match vault.steps.get(&next) {
                    Some(Step::Door(key)) => match (self.missing_keys_bitmask >> *key) & 1 {
                        0 => Some(State {
                            missing_keys_bitmask: self.missing_keys_bitmask,
                            position: next,
                        }),
                        _ => None,
                    },
                    Some(Step::Key(key)) => Some(State {
                        missing_keys_bitmask: match (self.missing_keys_bitmask >> *key) & 1 {
                            1 => self.missing_keys_bitmask ^ (1 << *key),
                            _ => self.missing_keys_bitmask,
                        },
                        position: next,
                    }),
                    Some(Step::Open) => Some(State {
                        missing_keys_bitmask: self.missing_keys_bitmask,
                        position: next,
                    }),
                    _ => None,
                }
            })
            .collect()
    }
}

fn get_step_count(vault: &Vault) -> Option<isize> {
    let steps: &mut HashMap<State, isize> = &mut HashMap::new();
    let state = State::from_vault(vault);
    let mut queue: VecDeque<State> = VecDeque::new();

    queue.push_back(state);
    steps.insert(state, 0);

    while let Some(state) = queue.pop_front() {
        match steps.get(&state) {
            Some(steps_taken) => {
                let step_count = *steps_taken;

                if state.missing_keys_bitmask == 0 {
                    return Some(step_count);
                }

                for next in state.next(vault) {
                    steps.entry(next).or_insert_with(|| {
                        queue.push_back(next);
                        step_count + 1
                    });
               }
            }
            _ => break,
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let vault = Vault::from_string(&input);
    println!(
        "How many steps is the shortest path that collects all of the keys? {}",
        get_step_count(&vault).unwrap(),
    );

    let mut steps = 0;
    for partition in vault.separate_sections() {
        steps += get_step_count(&partition).unwrap();
    }
    println!(
        "…what is the fewest steps necessary to collect all of the keys? {}",
        steps,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        let input = String::from(
            r#"#########
#b.A.@.a#
#########"#,
        );
        let vault = Vault::from_string(&input);
        print!("{:?}", vault);
        assert_eq!(8, get_step_count(&vault).unwrap());
    }

    #[test]
    fn test_86() {
        let input = String::from(
            r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"#,
        );
        let vault = Vault::from_string(&input);
        print!("{:?}", vault);
        assert_eq!(86, get_step_count(&vault).unwrap());
    }

    #[test]
    fn test_132() {
        let input = String::from(
            r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"#,
        );
        let vault = Vault::from_string(&input);
        print!("{:?}", vault);
        assert_eq!(132, get_step_count(&vault).unwrap());
    }

    #[test]
    fn test_136() {
        let input = String::from(
            r#"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"#,
        );
        let vault = Vault::from_string(&input);
        print!("{:?}", vault);
        assert_eq!(136, get_step_count(&vault).unwrap());
    }

    #[test]
    fn test_81() {
        let input = String::from(
            r#"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
"#,
        );
        let vault = Vault::from_string(&input);
        print!("{:?}", vault);
        assert_eq!(81, get_step_count(&vault).unwrap());
    }
}
