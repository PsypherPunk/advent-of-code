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
}

fn get_molecule_count(calibration: &Calibration) -> usize {
    let mut molecules = HashSet::new();

    for replacement in calibration.replacements.iter() {
        for (start, part) in calibration.molecule.match_indices(&replacement.from) {
            let mut molecule = String::new();
            molecule.push_str(&calibration.molecule[..start]);
            molecule.push_str(&replacement.to);
            molecule.push_str(&calibration.molecule[(start + part.len())..]);
            molecules.insert(molecule);
        }
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
}
