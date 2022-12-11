#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for AdventOfCodeError {
    fn from(error: ParseIntError) -> Self {
        AdventOfCodeError::ParseIntError(error)
    }
}

struct Cpu {
    x_register: isize,
    cycle: isize,
    signal_strengths: isize,
    screen: [bool; 40 * 6],
}

impl Cpu {
    fn new() -> Self {
        Self {
            x_register: 1,
            cycle: 1,
            signal_strengths: 0,
            screen: [false; 40 * 6],
        }
    }

    fn set_pixel(&mut self) {
        let column = (self.cycle - 1) % 40;
        self.screen[self.cycle as usize - 1] =
            (self.x_register - 1..=self.x_register + 1).contains(&column);
    }
}

pub fn get_part_one(input: &str) -> Result<isize, AdventOfCodeError> {
    let cpu = input.trim().lines().try_fold(Cpu::new(), |mut cpu, line| {
        if cpu.cycle % 40 == 20 {
            cpu.signal_strengths += cpu.cycle * cpu.x_register;
        }
        cpu.cycle += 1;

        if let Some(("addx", value)) = line.split_once(' ') {
            if cpu.cycle % 40 == 20 {
                cpu.signal_strengths += cpu.cycle * cpu.x_register;
            }

            cpu.x_register += value.parse::<isize>()?;
            cpu.cycle += 1;
        }

        Ok::<Cpu, AdventOfCodeError>(cpu)
    })?;

    Ok(cpu.signal_strengths)
}

pub fn get_part_two(input: &str) -> Result<String, AdventOfCodeError> {
    let cpu = input.trim().lines().try_fold(Cpu::new(), |mut cpu, line| {
        cpu.set_pixel();
        cpu.cycle += 1;

        if let Some(("addx", value)) = line.split_once(' ') {
            cpu.set_pixel();

            cpu.x_register += value.parse::<isize>()?;
            cpu.cycle += 1;
        }

        Ok::<Cpu, AdventOfCodeError>(cpu)
    })?;

    let display = cpu
        .screen
        .map(|pixel| match pixel {
            true => '#',
            false => '.',
        })
        .chunks(40)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(display)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGER_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;
    const IMAGE: &str = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(13_140), get_part_one(LARGER_INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(IMAGE.to_owned()), get_part_two(LARGER_INPUT));
    }
}
