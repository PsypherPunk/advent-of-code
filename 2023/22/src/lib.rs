use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .splitn(6, &[',', '~'])
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        match &numbers[..] {
            [ax, ay, az, bx, by, bz] => Ok(Self {
                x: (*ax, *bx),
                y: (*ay, *by),
                z: (*az, *bz),
            }),
            _ => Err(format!("invalid line: {}", s)),
        }
    }
}

impl Brick {
    fn covers(&self, other: &Self) -> bool {
        self.x.0.max(other.x.0) <= self.x.1.min(other.x.1)
            && self.y.0.max(other.y.0) <= self.y.1.min(other.y.1)
    }

    fn fall_to(&self, z: usize) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: (z, z + (self.z.1 - self.z.0)),
        }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut bricks = input
        .trim()
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    bricks.sort_unstable_by_key(|brick| brick.z.0);

    let bricks = bricks
        .iter()
        .enumerate()
        .fold(vec![], |mut beneath, (i, brick)| {
            let resting_z = beneath[..i].iter().fold(1, |min_z: usize, other| {
                if brick.covers(other) {
                    min_z.max(other.z.1 + 1)
                } else {
                    min_z
                }
            });
            beneath.push(brick.fall_to(resting_z));

            beneath
        });
    
    let below =bricks
        .iter()
        .enumerate()
        .map(|(i, brick)| {
            bricks[..i]
                .iter()
                .enumerate()
                .filter_map(|(j, other)| {
                    if brick.covers(other) && brick.z.0 == other.z.1 + 1 {
                        Some(j)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let unsafe_ = below
        .iter()
        .filter_map(|bricks_below| {
            match bricks_below.len() {
                1 => bricks_below.first(),
                _ => None,
            }
        })
        .collect::<HashSet<_>>();

    Ok(bricks.len() - unsafe_.len())
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let mut bricks = input
        .trim()
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    bricks.sort_unstable_by_key(|brick| brick.z.0);

    let bricks = bricks
        .iter()
        .enumerate()
        .fold(vec![], |mut beneath, (i, brick)| {
            let resting_z = beneath[..i].iter().fold(1, |min_z: usize, other| {
                if brick.covers(other) {
                    min_z.max(other.z.1 + 1)
                } else {
                    min_z
                }
            });
            beneath.push(brick.fall_to(resting_z));

            beneath
        });

    let mut above = vec![Vec::new(); bricks.len()];
    let mut below = vec![Vec::new(); bricks.len()];
    for i in 0..bricks.len() {
        for j in 0..i {
            if bricks[i].covers(&bricks[j]) && bricks[i].z.0 == bricks[j].z.1 + 1 {
                above[j].push(i);
                below[i].push(j);
            }
        }
    }

    let mut safe = vec![true; below.len()];
    for underneath in &below {
        if underneath.len() == 1 {
            safe[underneath[0]] = false;
        }
    }

    let mut chained = 0;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    for (start, safe) in safe.iter().enumerate() {
        if *safe {
            continue;
        }

        queue.push_back(start);
        seen.insert(start);

        while let Some(current) = queue.pop_front() {
            for &next in &above[current] {
                if below[next].iter().all(|brick| seen.contains(brick)) && seen.insert(next) {
                    chained += 1;
                    queue.push_back(next);
                }
            }
        }

        seen.clear();
    }

    Ok(chained)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(5), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(7), get_part_two(INPUT));
    }
}
