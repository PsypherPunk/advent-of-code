pub fn get_offsets(input: &str) -> Vec<isize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn get_steps_to_exit(mut offsets: Vec<isize>) -> usize {
    let mut steps = 0;
    let mut position: isize = 0;

    while position >= 0 && (position as usize) < offsets.len() {
        let jump = offsets[position as usize];
        offsets[position as usize] += 1;

        position += jump;
        steps += 1;
    }

    steps
}

pub fn get_stranger_steps_to_exit(mut offsets: Vec<isize>) -> usize {
    let mut steps = 0;
    let mut position: isize = 0;

    while position >= 0 && (position as usize) < offsets.len() {
        let jump = offsets[position as usize];

        offsets[position as usize] += match offsets[position as usize] {
            o if o >= 3 => -1,
            _ => 1,
        };

        position += jump;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"0
3
0
1
-3"#;

    #[test]
    fn test_part_one() {
        let offsets = get_offsets(&INPUT);

        assert_eq!(5, get_steps_to_exit(offsets));
    }

    #[test]
    fn test_part_two() {
        let offsets = get_offsets(&INPUT);

        assert_eq!(10, get_stranger_steps_to_exit(offsets));
    }
}
