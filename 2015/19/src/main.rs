use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

#[derive(Debug)]
struct Calibration {
    replacements: Vec<Replacement>,
    molecule: String,
}

impl Replacement {
    fn from_string(line: &str) -> Self {
        let from_to = line.trim().split(" => ").collect::<Vec<&str>>();

        Replacement {
            from: String::from(*from_to.first().unwrap()),
            to: String::from(*from_to.last().unwrap()),
        }
    }
}

impl Calibration {
    fn from_string(input: &str) -> Self {
        let parts = input.trim().split("\n\n").collect::<Vec<&str>>();

        Calibration {
            replacements: parts
                .first()
                .unwrap()
                .lines()
                .map(|line| Replacement::from_string(&line))
                .collect::<Vec<Replacement>>(),
            molecule: String::from(*parts.last().unwrap()),
        }
    }

    fn find_fewest_steps(&self, input: &str, depth: usize) -> Option<usize> {
        if input == "e" {
            return Some(depth);
        }
        for next in self.replacements.iter().flat_map(|replacement| {
            let reverse = Replacement {
                to: replacement.from.clone(),
                from: replacement.to.clone(),
            };
            make_replacement(input, &reverse).into_iter()
        }) {
            if let Some(count) = self.find_fewest_steps(&next, depth + 1) {
                return Some(count);
            }
        }
        None
    }
}

fn make_replacement(molecule: &str, replacement: &Replacement) -> HashSet<String> {
    let mut molecules = HashSet::new();

    for (start, part) in molecule.match_indices(&replacement.from) {
        let mut new = String::new();
        new.push_str(&molecule[..start]);
        new.push_str(&replacement.to);
        new.push_str(&molecule[(start + part.len())..]);
        molecules.insert(new);
    }

    molecules
}

fn get_molecule_count(calibration: &Calibration) -> usize {
    let mut molecules = HashSet::new();

    for replacement in calibration.replacements.iter() {
        let new = make_replacement(&calibration.molecule, &replacement);
        molecules.extend(new);
    }

    molecules.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let calibration = Calibration::from_string(&input);

    println!(
        "How many distinct molecules can be created…? {}",
        get_molecule_count(&calibration),
    );

    println!(
        "…what is the fewest number of steps to go from e to the medicine molecule? {}",
        calibration
            .find_fewest_steps(&calibration.molecule, 0)
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"H => HO
H => OH
O => HH

HOH"#;

        let calibration = Calibration::from_string(&input);

        assert_eq!(4, get_molecule_count(&calibration));
    }

    #[test]
    fn test_part2() {
        let input = r#"e => H
e => O
H => HO
H => OH
O => HH

HOH"#;

        let calibration = Calibration::from_string(&input);

        assert_eq!(
            3,
            calibration
                .find_fewest_steps(&calibration.molecule, 0)
                .unwrap()
        )
    }
}
