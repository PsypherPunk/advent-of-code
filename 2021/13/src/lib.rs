use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize),
}

fn get_instructions(input: &str) -> (Vec<(isize, isize)>, Vec<Fold>) {
    let (dots, folds) = input.trim().split_once("\n\n").unwrap();

    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>();

    let folds = folds
        .lines()
        .map(|line| {
            let (fold, position) = line.split_once('=').unwrap();
            let position = position.parse::<isize>().unwrap();
            match fold.chars().last().unwrap() {
                'x' => Fold::X(position),
                'y' => Fold::Y(position),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>();

    (dots, folds)
}

pub fn get_part_one(input: &str) -> usize {
    let (dots, folds) = get_instructions(input);

    dots.iter()
        .map(|&(x, y)| match folds[0] {
            Fold::X(position) => (position - (x - position).abs(), y),
            Fold::Y(position) => (x, position - (y - position).abs()),
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn get_part_two(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(17, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, get_part_two(INPUT));
    }
}
