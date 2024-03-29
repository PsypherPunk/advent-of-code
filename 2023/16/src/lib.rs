use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Down,
    Right,
    Up,
}

struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Beam {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
}

fn get_move(beam: Beam) -> Beam {
    let (dx, dy) = match beam.direction {
        Direction::Left => (-1, 0),
        Direction::Down => (0, 1),
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
    };

    Beam {
        x: (beam.x as isize + dx) as usize,
        y: (beam.y as isize + dy) as usize,
        direction: beam.direction,
    }
}

fn get_energized_tiles(contraption: &[Vec<char>], start: Beam) -> usize {
    let mut seen = vec![vec![[false; 4]; contraption[0].len()]; contraption.len()];
    let mut beams = VecDeque::new();
    beams.push_back(start);

    while let Some(beam) = beams.pop_front() {
        if beam.x >= contraption.len() || beam.y >= contraption[0].len() {
            continue;
        }
        if seen[beam.y][beam.x][beam.direction as usize] {
            continue;
        }

        seen[beam.y][beam.x][beam.direction as usize] = true;

        match (contraption[beam.y][beam.x], beam.direction) {
            ('.', _) => beams.push_back(get_move(beam)),
            ('/', _) => beams.push_back(get_move(Beam {
                x: beam.x,
                y: beam.y,
                direction: match beam.direction {
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Right,
                },
            })),
            ('\\', _) => beams.push_back(get_move(Beam {
                x: beam.x,
                y: beam.y,
                direction: match beam.direction {
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                },
            })),
            ('|', Direction::Up | Direction::Down) => beams.push_back(get_move(beam)),
            ('-', Direction::Left | Direction::Right) => beams.push_back(get_move(beam)),
            ('|', _) => {
                beams.push_back(get_move(Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Up,
                }));
                beams.push_back(get_move(Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Down,
                }));
            }
            ('-', _) => {
                beams.push_back(get_move(Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Left,
                }));
                beams.push_back(get_move(Beam {
                    x: beam.x,
                    y: beam.y,
                    direction: Direction::Right,
                }));
            }
            _ => unreachable!(),
        }
    }

    seen.iter()
        .flatten()
        .filter(|x| x.iter().any(|&b| b))
        .count()
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let contraption = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok(get_energized_tiles(
        &contraption,
        Beam {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    ))
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let contraption = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let energized_tiles = (0..contraption.len())
        .flat_map(|y| {
            [
                Beam::new(0, y, Direction::Right),
                Beam::new(contraption[0].len() - 1, y, Direction::Left),
            ]
        })
        .chain((0..contraption[0].len()).flat_map(|x| {
            [
                Beam::new(x, 0, Direction::Down),
                Beam::new(x, contraption.len() - 1, Direction::Up),
            ]
        }))
        .par_bridge()
        .map(|start| get_energized_tiles(&contraption, start))
        .max()
        .ok_or(format!("couldn't cound energized tiles: {}", input))?;

    Ok(energized_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(46), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(51), get_part_two(INPUT));
    }
}
