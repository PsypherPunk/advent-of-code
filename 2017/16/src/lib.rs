use std::collections::HashMap;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn get_final_positions(dancers: &[char], moves: &[Move]) -> Vec<char> {
    let mut dancers = dancers.to_vec();

    for dance_move in moves {
        match dance_move {
            Move::Spin(size) => {
                for _ in 0..*size {
                    let d = dancers.pop().unwrap();
                    dancers.insert(0, d);
                }
            }
            Move::Exchange(a, b) => {
                dancers.swap(*a, *b);
            }
            Move::Partner(a, b) => {
                let a = dancers.iter().position(|d| *d == *a).unwrap();
                let b = dancers.iter().position(|d| *d == *b).unwrap();
                dancers.swap(a, b);
            }
        }
    }

    dancers
}

fn get_dancers(number: usize) -> Vec<char> {
    (0..number)
        .map(|position| (position as u8 + (b'a')) as char)
        .collect()
}

fn get_moves(input: &str) -> Vec<Move> {
    input
        .trim()
        .split(',')
        .map(|dance_move| match dance_move.chars().next().unwrap() {
            's' => {
                let size = dance_move[1..].parse::<usize>().unwrap();
                Move::Spin(size)
            }
            'x' => {
                let (a, b) = match dance_move[1..].split('/').collect::<Vec<_>>()[..] {
                    [a, b] => (a, b),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                Move::Exchange(a.parse().unwrap(), b.parse().unwrap())
            }
            'p' => {
                let (a, b) = match dance_move[1..].split('/').collect::<Vec<_>>()[..] {
                    [a, b] => (a, b),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                Move::Partner(a.chars().next().unwrap(), b.chars().next().unwrap())
            }
            _ => {
                panic!(r#"¯\_(ツ)_/¯"#);
            }
        })
        .collect()
}

pub fn perform_dance(input: &str) -> String {
    perform_dances(16, &input, 1)
}

pub fn perform_dances(dancers: usize, input: &str, times: usize) -> String {
    let mut dancers = get_dancers(dancers);
    let moves = get_moves(&input);
    let mut seen = HashMap::new();

    for i in 0..times {
        dancers = get_final_positions(&dancers, &moves);
        let positions = dancers.iter().collect::<String>();

        if seen.contains_key(&positions) {
            let (k, _) = seen
                .iter()
                .find(|&(_, v): &(&String, &usize)| *v == (times % i) - 1)
                .unwrap();
            dancers = k.chars().collect::<Vec<_>>();
            break;
        }
        seen.insert(positions, i);
    }

    dancers.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dancers() {
        assert_eq!(vec!['a', 'b', 'c'], get_dancers(3));
    }

    #[test]
    fn test_get_final_positions() {
        let dancers = get_dancers(5);
        let moves = get_moves("s1,x3/4,pe/b");

        let final_positions = get_final_positions(&dancers, &moves);

        assert_eq!(vec!['b', 'a', 'e', 'd', 'c'], final_positions);
    }

    #[test]
    fn test_part_two() {
        let final_positions = perform_dances(5, "s1,x3/4,pe/b", 2);

        assert_eq!("ceadb".to_string(), final_positions);
    }
}
