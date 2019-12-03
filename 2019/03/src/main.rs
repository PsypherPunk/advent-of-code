use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

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

fn trace_wire(steps: Vec<&str>) -> Vec<(isize, isize)> {
    let mut locations: Vec<(isize, isize)> = Vec::new();

    let mut current: (isize, isize) = (0, 0);
    for step in steps {
        match step {
            step if step.starts_with("U") => {
                for y in (current.1 + 1)..=(current.1 + step[1..].parse::<isize>().unwrap()) {
                    current = (current.0, y);
                    locations.push(current);
                }
            },
            step if step.starts_with("R") => {
                for x in (current.0 + 1)..=(current.0 + step[1..].parse::<isize>().unwrap()) {
                    current = (x, current.1);
                    locations.push(current);
                }
            },
            step if step.starts_with("D") => {
                for y in ((current.1 - step[1..].parse::<isize>().unwrap())..=(current.1 - 1)).rev() {
                    current = (current.0, y);
                    locations.push(current);
                }
            },
            step if step.starts_with("L") => {
                for x in ((current.0 - step[1..].parse::<isize>().unwrap())..=(current.0 - 1)).rev() {
                    current = (x, current.1);
                    locations.push(current);
                }
            },
            _ => panic!("Invalid step: {}", step),
        };
    }

    locations
}

fn find_closest_intersection(input: &String) -> usize {
    let mut routes: Vec<HashSet<(isize, isize)>> = Vec::new();

    for line in input.lines() {
        let steps = line.split(",").collect::<Vec<&str>>();
        let route = HashSet::from_iter(trace_wire(steps));
        routes.push(route);
    }
    routes[0]
        .intersection(&routes[1])
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap() as usize
}

fn get_fewest_steps(input: String) -> usize {
    let mut routes: [Vec<(isize, isize)>; 2] = [Vec::new(), Vec::new()];
    let mut unique: [HashSet<(isize, isize)>; 2] = [HashSet::new(), HashSet::new()];

    for (i, line) in input.lines().enumerate() {
        let steps = line.split(",").collect::<Vec<&str>>();
        routes[i] = trace_wire(steps);
        unique[i] = HashSet::from_iter(routes[i].clone());
    }
    let common = unique[0].intersection(&unique[1]);
    common
        .map(|c| {
            routes[0].iter().position(|&s| s == *c).unwrap() +
                routes[1].iter().position(|&s| s == *c).unwrap() +
                2
        })
        .min()
        .unwrap()
}

fn main() {
    let input = read_input();
    let closest_intersection = find_closest_intersection(&input);
    println!("The Manhattan distance to the closest intersection is: {}", closest_intersection);

    let fewest_steps = get_fewest_steps(input);
    println!("What is the fewest combined steps the wires must take to reach an intersection? {}", fewest_steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let input = String::from(r#"R8,U5,L5,D3
U7,R6,D4,L4"#);
        assert_eq!(
            find_closest_intersection(&input),
            6,
        );
    }

    #[test]
    fn test_one() {
        let input = String::from(r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#);
        assert_eq!(
            find_closest_intersection(&input),
            159,
        );
    }

    #[test]
    fn test_two() {
        let input = String::from(r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#);
        assert_eq!(
            find_closest_intersection(&input),
            135,
        );
    }

    #[test]
    fn test_zero_steps() {
        let input = String::from(r#"R8,U5,L5,D3
U7,R6,D4,L4"#);
        assert_eq!(
            get_fewest_steps(input),
            30,
        );
    }

    #[test]
    fn test_three() {
        let input = String::from(r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#);
        assert_eq!(
            get_fewest_steps(input),
            610,
        );
    }

    #[test]
    fn test_four() {
        let input = String::from(r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#);
        assert_eq!(
            get_fewest_steps(input),
            410,
        );
    }
}
