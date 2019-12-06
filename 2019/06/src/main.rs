use std::collections::HashMap;
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

fn read_map(input: &String) -> HashMap<&str, &str> {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for orbit in input.lines() {
        let pair: Vec<&str> = orbit.split(")").collect();
        let satellite = *pair.last().unwrap();
        let obj = *pair.first().unwrap();
        orbits.insert(satellite, obj);
    }
    orbits
}

fn get_transfer(satellite: &str, distance: usize, orbits: &HashMap<&str, &str>) -> usize {
    if satellite == "COM" {
        distance
    } else {
        get_transfer(orbits.get(satellite).unwrap(), distance + 1, orbits)
    }
}

fn count_orbits(orbits: &HashMap<&str, &str>) -> usize {
    orbits.keys().map(|o| get_transfer(o, 0, orbits)).sum()
}

fn get_route<'a>(
    satellite: &'a str,
    orbits: &'a HashMap<&'a str, &'a str>,
    route: &'a mut Vec<&'a str>,
) -> &'a mut Vec<&'a str> {
    if satellite == "COM" {
        route
    } else {
        let next = orbits.get(satellite).unwrap();
        route.push(satellite);
        get_route(next, orbits, route)
    }
}

fn get_minimum_orbital_transfers(orbits: &HashMap<&str, &str>) -> usize {
    let mut you_route: Vec<&str> = Vec::new();
    let you_path = get_route("YOU", orbits, &mut you_route);

    let mut san_route: Vec<&str> = Vec::new();
    let san_path = get_route("SAN", orbits, &mut san_route);

    for (distance, obj) in you_path.iter().enumerate() {
        match san_path.iter().position(|&o| &o == obj) {
            Some(o) => return distance + o - 2,
            None => {}
        }
    }
    panic!("Couldn't find a route to Santa!");
}

fn main() {
    let input = read_input();
    let orbits = read_map(&input);
    let total_orbits = count_orbits(&orbits);
    println!(
        "What is the total number of direct and indirect orbits in your map data? {}",
        total_orbits
    );
    let minimum_orbital_transfers = get_minimum_orbital_transfers(&orbits);
    println!(
        "What is the minimum number of orbital transfers required…? {}",
        minimum_orbital_transfers
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = String::from(
            r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#,
        );
        let orbits = read_map(&input);
        assert_eq!(count_orbits(&orbits), 42);
    }

    #[test]
    fn test_two() {
        let input = String::from(
            r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#,
        );
        let orbits = read_map(&input);
        assert_eq!(get_minimum_orbital_transfers(&orbits), 4);
    }
}
