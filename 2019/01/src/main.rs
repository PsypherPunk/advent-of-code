use std::fs::File;
use std::io::prelude::*;

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

fn get_fuel_for_module(mass: isize) -> isize {
    (mass / 3) - 2
}

fn get_total_fuel_requirement(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .map(get_fuel_for_module)
        .sum()
}

fn get_fuel_for_module_including_fuel(mass: isize) -> isize {
    let fuel_for_mass = (mass / 3) - 2;
    match fuel_for_mass {
        fuel if fuel <= 0 => 0,
        _ => fuel_for_mass + get_fuel_for_module_including_fuel(fuel_for_mass),
    }
}

fn get_total_fuel_requirement_including_fuel(input: String) -> isize {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .map(get_fuel_for_module_including_fuel)
        .sum()
}

fn main() {
    let input = read_input();
    let total_fuel_requirement = get_total_fuel_requirement(&input);
    let total_fuel_requirement_including_fuel = get_total_fuel_requirement_including_fuel(input);
    println!(
        "What is the sum of the fuel requirements… {}",
        total_fuel_requirement,
    );
    println!(
        "What is the sum of the fuel requirements… {}",
        total_fuel_requirement_including_fuel,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(get_fuel_for_module(12), 2);
    }

    #[test]
    fn test_14() {
        assert_eq!(get_fuel_for_module(14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(get_fuel_for_module(1969), 654);
    }

    #[test]
    fn test_100756() {
        assert_eq!(get_fuel_for_module(100756), 33583);
    }

    #[test]
    fn test_14_inc_fuel() {
        assert_eq!(get_fuel_for_module_including_fuel(14), 2);
    }

    #[test]
    fn test_1969_inc_fuel() {
        assert_eq!(get_fuel_for_module_including_fuel(1969), 966);
    }

    #[test]
    fn test_100756_inc_fuel() {
        assert_eq!(get_fuel_for_module_including_fuel(100756), 50346);
    }
}
