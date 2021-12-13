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

pub fn get_part_two(input: &str) -> String {
    let (dots, folds) = get_instructions(input);

    let dots = dots
        .iter()
        .map(|&(x, y)| {
            folds.iter().fold((x, y), |(x, y), fold| match fold {
                Fold::X(position) => (position - (x - position).abs(), y),
                Fold::Y(position) => (x, position - (y - position).abs()),
            })
        })
        .collect::<HashSet<_>>();

    let (width, height) = folds
        .iter()
        .fold((usize::MAX, usize::MAX), |(x, y), fold| match fold {
            Fold::X(position) => (x.min(*position as usize), y),
            Fold::Y(position) => (x, y.min(*position as usize)),
        });

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| match dots.contains(&(x as isize, y as isize)) {
                    true => '⬛',
                    false => '⬜',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
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
        let output = r#"#####
#...#
#...#
#...#
#####
.....
....."#
            .replace('.', "⬜")
            .replace('#', "⬛")
            .to_owned();

        assert_eq!(output, get_part_two(INPUT));
    }
}
