use std::collections::{HashMap, HashSet};

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut lines = input.lines();

    let start = lines
        .next()
        .ok_or("invalid input")?
        .find('S')
        .ok_or("no start found")?;

    let mut splits = 0;
    let mut beams = HashSet::from([start]);

    for line in lines {
        let mut new_beams = beams.clone();

        for &position in &beams {
            if let Some('^') = line.chars().nth(position) {
                splits += 1;
                new_beams.insert(position - 1);
                new_beams.insert(position + 1);
                new_beams.remove(&position);
            }
        }

        beams = new_beams;
    }

    Ok(splits)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut lines = input.lines();

    let start = lines
        .next()
        .ok_or("invalid input")?
        .find('S')
        .ok_or("no start found")?;

    let mut timelines = HashMap::from([(start, 1)]);

    for line in lines {
        let mut new_timelines = timelines.clone();

        for (&position, &count) in &timelines {
            if let Some('^') = line.chars().nth(position) {
                *new_timelines.entry(position - 1).or_insert(0) += count;
                *new_timelines.entry(position + 1).or_insert(0) += count;
                new_timelines.remove(&position);
            }
        }

        timelines = new_timelines;
    }

    Ok(timelines.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(21), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(40), get_part_two(INPUT));
    }
}
