use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Instructions {
    dependents: HashMap<char, Vec<char>>,
    timings: HashMap<char, usize>,
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dependents = HashMap::new();
        let mut timings = HashMap::new();

        s.trim().lines().for_each(|line| {
            let chars = line.chars().collect::<Vec<_>>();

            dependents.entry(chars[36]).or_insert_with(Vec::new);
            dependents.entry(chars[5]).or_default().push(chars[36]);

            timings
                .entry(chars[5])
                .or_insert_with(|| chars[5] as usize - ('A' as usize) + 61);
            timings
                .entry(chars[36])
                .or_insert_with(|| chars[36] as usize - ('A' as usize) + 61);
        });

        Ok(Self {
            dependents,
            timings,
        })
    }
}

impl Instructions {
    /// Get the order of steps for the instructions.
    ///
    /// Find the step which does not occur in any other step's
    /// dependents, then remove this causing its own dependents to
    /// occur in one fewer set of dependents.
    pub fn get_steps_order(&self) -> String {
        let mut steps = self.dependents.clone();
        let mut output = Vec::new();

        while let Some(&step) = steps
            .iter()
            .map(|(step, _)| {
                let count = steps
                    .values()
                    .map(|dependents| dependents.contains(step) as usize)
                    .sum::<usize>();
                (step, count)
            })
            .filter(|(_, count)| *count == 0)
            .map(|(step, _)| step)
            .min()
        {
            output.push(step);
            steps.remove(&step);
        }

        output.into_iter().collect()
    }

    /// Get the duration for all steps.
    ///
    /// Again, we first find the ``unblocked`` steps (i.e. those with
    /// no preceding requirements), before ticking over each second and
    /// reducing the relevant timings.
    pub fn get_duration(&mut self, workers: usize) -> usize {
        let mut seconds = 0;

        while !self.timings.is_empty() {
            let steps = self.dependents.clone();
            let mut unblocked = steps
                .iter()
                .map(|(step, _)| {
                    let count = steps
                        .values()
                        .filter(|dependents| dependents.contains(&step))
                        .count();
                    (step, count)
                })
                .collect::<Vec<_>>();
            unblocked.sort_by(|a, b| b.1.cmp(&a.1));

            let unblocked = unblocked
                .into_iter()
                .filter(|(_, count)| *count == 0)
                .map(|(step, _)| step)
                .collect::<Vec<_>>();

            for step in unblocked.iter().take(workers) {
                let count = self.timings.get_mut(&step).unwrap();
                *count -= 1;
                if *count == 0 {
                    self.timings.remove(&step);
                    self.dependents.remove(&step);
                }
            }

            seconds += 1;
        }

        seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;

    #[test]
    fn test_part_one() {
        let instructions = Instructions::from_str(&INPUT).unwrap();

        assert_eq!("CABDFE", instructions.get_steps_order());
    }

    #[test]
    fn test_part_two() {
        let mut instructions = Instructions::from_str(&INPUT).unwrap();

        instructions
            .timings
            .iter_mut()
            .for_each(|(_, timing)| *timing -= 60);

        assert_eq!(15, instructions.get_duration(2));
    }
}
