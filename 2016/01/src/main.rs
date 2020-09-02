use std::collections::HashSet;
use std::fs;

type Point = (isize, isize);

#[derive(Debug)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Position {
    point: Point,
    facing: Cardinal,
    visited: Vec<Point>,
}

struct Instruction {
    direction: Direction,
    blocks: isize,
}

impl Position {
    fn new() -> Self {
        let mut position = Position {
            point: (0, 0),
            facing: Cardinal::North,
            visited: Vec::new(),
        };
        position.visited.push((0, 0));

        position
    }

    fn turn(&mut self, direction: &Direction) {
        let left = *direction == Direction::Left;
        self.facing = match self.facing {
            Cardinal::North => {
                if left {
                    Cardinal::West
                } else {
                    Cardinal::East
                }
            }
            Cardinal::East => {
                if left {
                    Cardinal::North
                } else {
                    Cardinal::South
                }
            }
            Cardinal::South => {
                if left {
                    Cardinal::East
                } else {
                    Cardinal::West
                }
            }
            Cardinal::West => {
                if left {
                    Cardinal::South
                } else {
                    Cardinal::North
                }
            }
        }
    }

    fn advance(&mut self, blocks: isize) {
        match self.facing {
            Cardinal::North => {
                for _ in 1..=blocks {
                    self.point.1 += 1;
                    self.visited.push(self.point);
                }
            }
            Cardinal::East => {
                for _ in 1..=blocks {
                    self.point.0 += 1;
                    self.visited.push(self.point);
                }
            }
            Cardinal::South => {
                for _ in 1..=blocks {
                    self.point.1 -= 1;
                    self.visited.push(self.point);
                }
            }
            Cardinal::West => {
                for _ in 1..=blocks {
                    self.point.0 -= 1;
                    self.visited.push(self.point);
                }
            }
        }
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split(", ")
        .map(|instruction| {
            let (direction, blocks) = instruction.split_at(1);
            let direction = match direction {
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("Invalid direction!"),
            };
            let blocks = blocks.parse::<isize>().unwrap();
            Instruction { direction, blocks }
        })
        .collect()
}

fn reposition(position: &mut Position, instruction: &Instruction) {
    position.turn(&instruction.direction);
    position.advance(instruction.blocks);
}

fn get_final_position(instructions: &[Instruction]) -> Position {
    let mut position = Position::new();

    for instruction in instructions {
        reposition(&mut position, instruction);
    }

    position
}

fn get_first_revisited(position: &Position) -> Point {
    let mut visited = HashSet::new();
    for point in &position.visited {
        if visited.contains(point) {
            return *point;
        } else {
            visited.insert(*point);
        }
    }
    panic!("Didn't find a revisit!");
}

fn get_final_distance(instructions: &[Instruction]) -> isize {
    let position = get_final_position(instructions);

    get_manhattan_distance(&position.point)
}

fn get_manhattan_distance(point: &Point) -> isize {
    let (x, y) = point;

    x.abs() + y.abs()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let instructions = get_instructions(&input);

    println!(
        "How many blocks away is Easter Bunny HQ? {}",
        get_final_distance(&instructions),
    );

    let position = get_final_position(&instructions);
    println!(
        "How many blocks away is the first location you visit twice? {}",
        get_manhattan_distance(&get_first_revisited(&position)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r2_l3() {
        let instructions = get_instructions("R2, L3");
        assert_eq!(5, get_final_distance(&instructions));
    }

    #[test]
    fn test_r2_r2_r2() {
        let instructions = get_instructions("R2, R2, R2");
        assert_eq!(2, get_final_distance(&instructions));
    }

    #[test]
    fn test_r5_l5_r5_r3() {
        let instructions = get_instructions("R5, L5, R5, R3");
        assert_eq!(12, get_final_distance(&instructions));
    }

    #[test]
    fn test_r8_r4_r4_r8() {
        let instructions = get_instructions("R8, R4, R4, R8");
        let position = get_final_position(&instructions);
        assert_eq!(4, get_manhattan_distance(&get_first_revisited(&position)));
    }
}
