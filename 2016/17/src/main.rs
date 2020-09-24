use std::collections::VecDeque;
use std::fs;

use md5::{Digest, Md5};

type Room = (isize, isize);

#[derive(Debug)]
struct Step {
    room: Room,
    passcode: String,
    path: String,
}

impl Step {
    const OPEN: [char; 5] = ['b', 'c', 'd', 'e', 'f'];

    fn new(passcode: &str, path: &str, room: Room) -> Self {
        Self {
            room,
            passcode: passcode.to_string(),
            path: path.to_string(),
        }
    }

    fn get_steps(&self) -> Vec<Step> {
        let mut hasher = Md5::new();
        hasher.update(self.passcode.as_bytes());
        hasher.update(self.path.as_bytes());

        let result = &hasher.finalize()[..];
        let udlr = hex::encode(result)
            .chars()
            .take(4)
            .map(|ch| Step::OPEN.contains(&ch))
            .zip(['U', 'D', 'L', 'R'].iter())
            .collect::<Vec<(bool, &char)>>();

        [
            (self.room.0, self.room.1 - 1),
            (self.room.0, self.room.1 + 1),
            (self.room.0 - 1, self.room.1),
            (self.room.0 + 1, self.room.1),
        ]
        .iter()
        .zip(udlr)
        .filter(|&(_, (open, _))| open)
        .map(|(&point, (_, dir))| (point, dir))
        .filter(|((x, y), _)| *x >= 0 && *x <= 3 && *y >= 0 && *y <= 3)
        .map(|(room, dir)| {
            let mut path = self.path.clone();
            path.push(*dir);
            Step {
                room,
                passcode: self.passcode.clone(),
                path,
            }
        })
        .collect()
    }
}

fn get_shortest_path(passcode: &str) -> String {
    let mut queue = VecDeque::new();
    let start = Step::new(passcode, "", (0, 0));
    queue.push_back(start);

    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();

        if step.room == (3, 3) {
            return step.path;
        }

        for next_step in step.get_steps() {
            queue.push_back(next_step);
        }
    }

    panic!("Could not find path.");
}

fn get_longest_path_length(passcode: &str) -> usize {
    let mut queue = VecDeque::new();
    let start = Step::new(passcode, "", (0, 0));
    queue.push_back(start);
    let mut paths = Vec::new();

    while !queue.is_empty() {
        print!("{}\r", queue.len());
        let step = queue.pop_front().unwrap();

        if step.room == (3, 3) {
            paths.push(step.path.len());
            continue;
        }

        for next_step in step.get_steps() {
            queue.push_back(next_step);
        }
    }

    *paths.iter().max().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…what is the shortest path…to reach the vault? {}",
        get_shortest_path(&input.trim()),
    );

    println!(
        "What is the length of the longest path that reaches the vault? {}",
        get_longest_path_length(&input.trim()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ihgpwlah() {
        let input = "ihgpwlah";

        assert_eq!("DDRRRD", get_shortest_path(input));
    }

    #[test]
    fn test_kglvqrro() {
        let input = "kglvqrro";

        assert_eq!("DDUDRLRRUDRD", get_shortest_path(input));
    }

    #[test]
    fn test_ulqzkmiv() {
        let input = "ulqzkmiv";

        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", get_shortest_path(input));
    }

    #[test]
    fn test_ihgpwlah_length() {
        let input = "ihgpwlah";

        assert_eq!(370, get_longest_path_length(input));
    }

    #[test]
    fn test_kglvqrro_length() {
        let input = "kglvqrro";

        assert_eq!(492, get_longest_path_length(input));
    }

    #[test]
    fn test_ulqzkmiv_length() {
        let input = "ulqzkmiv";

        assert_eq!(830, get_longest_path_length(input));
    }
}
