use std::fs::File;
use std::io::prelude::*;

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        },
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn get_fuel_for_module_of(mass: usize) -> usize {
    (mass / 3) - 2
}

fn get_total_fuel_requirement(input: String) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mass| get_fuel_for_module_of(mass))
        .sum()
}

fn main() {
    let input = read_input();
    let total_fuel_requirement = get_total_fuel_requirement(input);
    println!("The sum of the fuel requirements is {}.", total_fuel_requirement);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(get_fuel_for_module_of(12), 2);
    }

    #[test]
    fn test_14() {
        assert_eq!(get_fuel_for_module_of(14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(get_fuel_for_module_of(1969), 654);
    }

    #[test]
    fn test_100756() {
        assert_eq!(get_fuel_for_module_of(100756), 33583);
    }
}
