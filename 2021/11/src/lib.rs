use std::collections::HashSet;
use std::{thread, time};

fn get_octopodes(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

fn get_neighbours(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y.saturating_sub(1)),
        (x + 1, y.saturating_sub(1)),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x.saturating_sub(1), y + 1),
        (x.saturating_sub(1), y),
        (x.saturating_sub(1), y.saturating_sub(1)),
    ]
    .into_iter()
    .filter(|&(x, y)| x < width && y < height)
    .collect::<HashSet<_>>()
    .into_iter()
    .collect()
}

fn flash_ah_aaaaaah(x: usize, y: usize, octopodes: &mut [Vec<i8>]) -> usize {
    let mut flashes = 1;

    // TODO: ugly sentinel value; use Option<i8>?
    octopodes[y][x] = -1;

    for (nx, ny) in get_neighbours(x, y, octopodes[0].len(), octopodes.len()) {
        if octopodes[ny][nx] != -1 {
            octopodes[ny][nx] += 1;
            if octopodes[ny][nx] > 9 {
                flashes += flash_ah_aaaaaah(nx, ny, octopodes);
            }
        }
    }

    flashes
}

fn display_step(octopodes: &[Vec<i8>]) {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    octopodes.iter().for_each(|row| {
        row.iter().for_each(|octopus| {
            let output = match octopus {
                0 => '💥',
                _ => '🐙',
            };

            print!("{}", output);
        });
        println!();
    });
    thread::sleep(time::Duration::from_millis(100));
}

pub fn get_part_one(input: &str, display: bool) -> usize {
    let mut octopodes = get_octopodes(input);
    let mut flash_count = 0;

    let height = octopodes.len();
    let width = octopodes[0].len();

    for _ in 1..=100 {
        octopodes.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|octopus| *octopus += 1);
        });

        for y in 0..height {
            for x in 0..width {
                if octopodes[y][x] > 9 {
                    flash_count += flash_ah_aaaaaah(x, y, &mut octopodes);
                }
            }
        }
        octopodes.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|octopus| *octopus == &-1)
                .for_each(|octopus| *octopus = 0);
        });

        if display {
            display_step(&octopodes);
        }
    }

    flash_count
}

pub fn get_part_two(input: &str, display: bool) -> usize {
    let mut octopodes = get_octopodes(input);
    let mut sync_step = 0;

    let height = octopodes.len();
    let width = octopodes[0].len();

    for step in 1.. {
        let mut flash_count = 0;
        octopodes.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|octopus| *octopus += 1);
        });

        for y in 0..height {
            for x in 0..width {
                if octopodes[y][x] > 9 {
                    flash_count += flash_ah_aaaaaah(x, y, &mut octopodes);
                }
            }
        }
        if flash_count == width * height {
            sync_step = step;
            break;
        }
        octopodes.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|octopus| *octopus == &-1)
                .for_each(|octopus| *octopus = 0);
        });

        if display {
            display_step(&octopodes);
        }
    }

    sync_step
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(1656, get_part_one(INPUT, false));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(195, get_part_two(INPUT, false));
    }
}
