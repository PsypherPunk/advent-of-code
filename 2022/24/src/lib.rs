use std::collections::HashSet;

pub fn get_part_one(input: &str) -> usize {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    let walls = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as isize, y as isize)),
                _ => None,
            })
        })
        .collect::<HashSet<_>>();

    let mut blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| ">v<^".contains(c).then_some((x as isize, y as isize, c)))
        })
        .collect::<Vec<_>>();

    let mut expeditions = HashSet::from_iter([(1, 0)]);

    let mut answer = 0;
    let goal = (width - 2, height - 1);

    for minutes in 1.. {
        for blizzard in &mut blizzards {
            match blizzard.2 {
                '>' => {
                    blizzard.0 = if blizzard.0 == width - 2 {
                        1
                    } else {
                        blizzard.0 + 1
                    }
                }
                '<' => {
                    blizzard.0 = if blizzard.0 == 1 {
                        width - 2
                    } else {
                        blizzard.0 - 1
                    }
                }
                'v' => {
                    blizzard.1 = if blizzard.1 == height - 2 {
                        1
                    } else {
                        blizzard.1 + 1
                    }
                }
                '^' => {
                    blizzard.1 = if blizzard.1 == 1 {
                        height - 2
                    } else {
                        blizzard.1 - 1
                    }
                }
                _ => unreachable!(),
            }
        }
        let blizzard_positions = blizzards
            .iter()
            .map(|&(x, y, _)| (x, y))
            .collect::<HashSet<_>>();

        let mut next_positions = HashSet::with_capacity(expeditions.len());

        for &(x, y) in &expeditions {
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)] {
                let (x, y) = (x + dx, y + dy);
                if (y > 0)
                    && (y < height)
                    && !walls.contains(&(x, y))
                    && !blizzard_positions.contains(&(x, y))
                {
                    next_positions.insert((x, y));
                }
            }
        }
        expeditions = next_positions;

        if expeditions.contains(&goal) {
            answer = minutes;
            break;
        }
    }

    answer
}

pub fn get_part_two(input: &str) -> usize {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    let walls = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as isize, y as isize)),
                _ => None,
            })
        })
        .collect::<HashSet<_>>();

    let mut blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| ">v<^".contains(c).then_some((x as isize, y as isize, c)))
        })
        .collect::<Vec<_>>();

    let mut expeditions = HashSet::from_iter([(1, 0)]);

    let mut answer = 0;
    let mut goal = (width - 2, height - 1);
    let mut got_snacks = false;

    for minutes in 1.. {
        for blizzard in &mut blizzards {
            match blizzard.2 {
                '>' => {
                    blizzard.0 = if blizzard.0 == width - 2 {
                        1
                    } else {
                        blizzard.0 + 1
                    }
                }
                '<' => {
                    blizzard.0 = if blizzard.0 == 1 {
                        width - 2
                    } else {
                        blizzard.0 - 1
                    }
                }
                'v' => {
                    blizzard.1 = if blizzard.1 == height - 2 {
                        1
                    } else {
                        blizzard.1 + 1
                    }
                }
                '^' => {
                    blizzard.1 = if blizzard.1 == 1 {
                        height - 2
                    } else {
                        blizzard.1 - 1
                    }
                }
                _ => unreachable!(),
            }
        }
        let blizzard_positions = blizzards
            .iter()
            .map(|&(x, y, _)| (x, y))
            .collect::<HashSet<_>>();

        let mut next_positions = HashSet::with_capacity(expeditions.len());

        for &(x, y) in &expeditions {
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)] {
                let (x, y) = (x + dx, y + dy);
                if (y > 0 || x == 1)
                    && (y < height || x == width - 2)
                    && !walls.contains(&(x, y))
                    && !blizzard_positions.contains(&(x, y))
                // don't move into a blizzard.
                {
                    next_positions.insert((x, y));
                }
            }
        }
        expeditions = next_positions;

        if expeditions.contains(&goal) {
            if got_snacks {
                answer = minutes;
                break;
            }
            if goal == (1, 0) {
                got_snacks = true;
                goal = (width - 2, height - 1);
                expeditions = HashSet::from_iter([(1, 0)]);
            } else {
                goal = (1, 0);
                expeditions = HashSet::from_iter([(width - 2, height - 1)]);
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(18, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(54, get_part_two(INPUT));
    }
}
