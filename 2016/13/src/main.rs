use std::collections::{HashSet, VecDeque};
use std::fs;

type Point = (usize, usize);

#[derive(Clone, Eq, Hash, PartialEq)]
struct Step {
    position: Point,
    steps: usize,
}

fn get_set_bit_count(num: usize) -> usize {
    let mut count = 0;
    let mut n = num;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }

    count
}

fn is_open_space(position: &Point, favourite: &usize) -> bool {
    let calc = (position.0 * position.0)
        + (3 * position.0)
        + (2 * position.0 * position.1)
        + (position.1)
        + (position.1 * position.1)
        + favourite;

    get_set_bit_count(calc) % 2 == 0
}

fn get_steps(previous: Step) -> Vec<Step> {
    let mut steps = vec![
        Step {
            position: (previous.position.0 + 1, previous.position.1),
            steps: previous.steps + 1,
        },
        Step {
            position: (previous.position.0, previous.position.1 + 1),
            steps: previous.steps + 1,
        },
    ];
    if previous.position.0 > 0 {
        steps.push(Step {
            position: (previous.position.0 - 1, previous.position.1),
            steps: previous.steps + 1,
        });
    }
    if previous.position.1 > 0 {
        steps.push(Step {
            position: (previous.position.0, previous.position.1 - 1),
            steps: previous.steps + 1,
        });
    }

    steps
}

fn get_shortest_route(favourite: &str, destination: Point) -> usize {
    let mut queue = VecDeque::new();
    let mut discovered = HashSet::new();
    let favourite = favourite.trim().parse::<usize>().unwrap();

    let root = Step {
        position: (1, 1),
        steps: 0,
    };
    discovered.insert(root.clone());
    queue.push_back(root);

    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        if step.position == destination {
            return step.steps;
        }

        for next in get_steps(step)
            .iter()
            .filter(|&next| is_open_space(&next.position, &favourite))
        {
            if !discovered.contains(next) {
                queue.push_back((*next).clone());
                discovered.insert(next.clone());
            }
        }
    }

    panic!("Unable to find destination!");
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the fewest number of steps required for you to reach 31,39? {}",
        get_shortest_route(&input, (31, 39)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_bit_count() {
        assert_eq!(2, get_set_bit_count(10));
        assert_eq!(2, get_set_bit_count(9));
        assert_eq!(1, get_set_bit_count(8));
        assert_eq!(3, get_set_bit_count(7));
    }

    #[test]
    fn test_7_4() {
        let favourite = "10";

        assert_eq!(11, get_shortest_route(favourite, (7, 4)));
    }
}
