use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::thread;
use std::time::Duration;

struct Screen {
    pixels: Vec<Vec<bool>>,
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_render())
    }
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            pixels: vec![vec![false; width]; height],
        }
    }

    fn get_render(&self) -> String {
        self.pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| match pixel {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn rect(&mut self, width: usize, height: usize) {
        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                self.pixels[y][x] = true;
            })
        });
    }

    fn rotate_column(&mut self, column: usize, rotation: usize) {
        let mut new = self
            .pixels
            .iter()
            .map(|row| row[column])
            .collect::<Vec<bool>>();

        (0..rotation).for_each(|_| {
            let wrap = new.pop().unwrap();
            new.insert(0, wrap);
        });

        self.pixels.iter_mut().enumerate().for_each(|(i, row)| {
            row[column] = new[i];
        })
    }

    fn rotate_row(&mut self, row: usize, rotation: usize) {
        let mut new = self.pixels[row].clone();

        (0..rotation).for_each(|_| {
            let wrap = new.pop().unwrap();
            new.insert(0, wrap);
        });

        new.iter()
            .enumerate()
            .for_each(|(i, pixel)| self.pixels[row][i] = *pixel);
    }

    fn read_instructions(&mut self, input: &str) {
        input.trim().lines().for_each(|line| {
            let words = line.trim().split_whitespace().collect::<Vec<_>>();

            match words[0..=1] {
                ["rect", _] => {
                    let dimensions = words[1].split('x').collect::<Vec<_>>();
                    let (width, height) = (
                        dimensions[0].parse::<usize>().unwrap(),
                        dimensions[1].parse::<usize>().unwrap(),
                    );
                    self.rect(width, height);
                }
                ["rotate", "column"] => {
                    let column = words[2].split('=').last().unwrap().parse().unwrap();
                    let rotation = words[4].parse().unwrap();
                    self.rotate_column(column, rotation);
                }
                ["rotate", "row"] => {
                    let row = words[2].split('=').last().unwrap().parse().unwrap();
                    let rotation = words[4].parse().unwrap();
                    self.rotate_row(row, rotation);
                }
                _ => panic!("Invalid instruction: {}", line),
            }
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", self);
            thread::sleep(Duration::from_millis(50));
        });
    }

    fn get_lit_count(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| match pixel {
                        true => 1,
                        false => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut screen = Screen::new(50, 6);
    print!("{}", screen);
    screen.read_instructions(&input);

    println!("…how many pixels should be lit? {}", screen.get_lit_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_3x2() {
        let mut screen = Screen::new(7, 3);

        let input = "rect 3x2";
        screen.read_instructions(&input);
        assert_eq!(
            r#"###....
###....
......."#,
            screen.get_render()
        );

        let input = "rotate column x=1 by 1";
        screen.read_instructions(&input);
        assert_eq!(
            r#"#.#....
###....
.#....."#,
            screen.get_render()
        );

        let input = "rotate row y=0 by 4";
        screen.read_instructions(&input);
        assert_eq!(
            r#"....#.#
###....
.#....."#,
            screen.get_render()
        );
    }
}
