fn get_spin(dancers: &[char], size: usize) -> Vec<char> {
    let split = dancers.len() - size;
    let mut new = dancers[split..].to_vec();
    new.extend_from_slice(&dancers[0..split]);

    new
}

fn get_final_positions(dancers: &[char], moves: Vec<&str>) -> Vec<char> {
    let mut dancers = dancers.to_vec();

    for dance_move in moves {
        match dance_move.chars().next().unwrap() {
            's' => {
                let size = dance_move[1..].parse::<usize>().unwrap();
                dancers = get_spin(&dancers, size);
            }
            'x' => {
                let (a, b) = match dance_move[1..].split('/').collect::<Vec<_>>()[..] {
                    [a, b] => (a, b),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                dancers.swap(a.parse().unwrap(), b.parse().unwrap());
            }
            'p' => {
                let (a, b) = match dance_move[1..].split('/').collect::<Vec<_>>()[..] {
                    [a, b] => (a, b),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                let a = dancers
                    .iter()
                    .position(|d| *d == a.chars().next().unwrap())
                    .unwrap();
                let b = dancers
                    .iter()
                    .position(|d| *d == b.chars().next().unwrap())
                    .unwrap();
                dancers.swap(a, b);
            }
            _ => {
                panic!(r#"¯\_(ツ)_/¯"#);
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

pub fn perform_dance(input: &str) -> String {
    let dancers = get_dancers(16);
    let moves = input.trim().split(',').collect();

    get_final_positions(&dancers, moves).into_iter().collect()
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
        let mut dancers = get_dancers(5);
        let moves = vec!["s1", "x3/4", "pe/b"];

        let final_positions = get_final_positions(&dancers, moves);

        assert_eq!(vec!['b', 'a', 'e', 'd', 'c'], final_positions);
    }
}
